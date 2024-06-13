use std::{fmt::Display, str::FromStr};

use crate::{
    ast1::{
        chunkvariant::ChunkVariant, command::Command, scope::Scope, scopevariant::ScopeVariant,
    },
    ast2,
};

use super::{chunk::Chunk, into_chunks::IntoChunks};

/// Main struct for stage 1 AST
///
/// Display `{}` reconstructs the original document
#[derive(Default, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Document(Vec<Chunk>);

impl Document {
    /// Return all the chunks within the document
    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.0
    }

    /// Return all the owned chunks within the document
    pub fn chunks_owned(self) -> Vec<Chunk> {
        self.0
    }

    /// Create new document from chunks
    pub fn new(chunks: Vec<Chunk>) -> Self {
        Self(chunks)
    }

    /// Push new chunks into document
    pub fn push(&mut self, mut chunks: Vec<Chunk>) {
        self.0.append(&mut chunks)
    }

    /// Push new chunks into document with structs that implement `ast1::IntoChunks`
    pub fn push_into<T: IntoChunks>(&mut self, original: T) {
        self.push(original.into_chunks())
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter().map(ToString::to_string).collect::<String>())
    }
}

impl From<ast2::Document> for Document {
    fn from(value: ast2::Document) -> Self {
        let mut out = Self::default();

        for chunk in value.chunks_owned() {
            out.push_into(chunk)
        }

        out
    }
}

/// Cursor buffer type
#[derive(Clone)]
enum Buffer {
    /// Currently in a scope,
    /// reading in characters until the scope is closed,
    /// then it will be parsed
    Scope {
        content: String,
        variant: ScopeVariant,
        depth: u32,
    },
    /// Currently in a command
    /// Enters this mode after a character is escaped
    /// Exit this mode when it hits a Text element
    /// note that following scopes will just be parsed as arguments for the command
    Command {
        label: String,
        scopes: Vec<(String, ScopeVariant, String)>, // content, variant, trailing
        depth: u32,
        trailing: String,
    },
    /// Stores all read characters in a buffer
    /// exit this mode when it hits a scope or command
    Text { content: String },
}

impl Buffer {
    pub fn no_scope(&self) -> bool {
        match self {
            Self::Command { scopes, .. } => scopes.is_empty(),
            _ => unreachable!("not a command")
        }
    }

    pub fn scope(variant: ScopeVariant) -> Self {
        Self::Scope {
            content: String::new(),
            depth: 1,
            variant,
        }
    }

    pub fn command() -> Self {
        Self::Command {
            label: String::new(),
            scopes: Vec::new(),
            depth: 0,
            trailing: String::new(),
        }
    }

    pub fn text() -> Self {
        Self::Text {
            content: String::new(),
        }
    }

    /// push a char to buffer
    pub fn push(&mut self, c: char) {
        match self {
            Self::Scope { content, .. } => content.push(c),
            Self::Command { label, scopes, .. } if scopes.is_empty() => label.push(c),
            Self::Command { scopes, .. } => scopes.last_mut().unwrap().0.push(c),
            Self::Text { content } => content.push(c),
        }
    }

    /// push a str to buffer
    pub fn push_str(&mut self, s: &str) {
        match self {
            Self::Scope { content, .. } => content.push_str(s),
            Self::Command { label, scopes, .. } if scopes.is_empty() => label.push_str(s),
            Self::Command { scopes, .. } => scopes.last_mut().unwrap().0.push_str(s),
            Self::Text { content } => content.push_str(s),
        }
    }

    /// push a scope to command arguments
    pub fn push_scope(&mut self, variant: ScopeVariant) {
        match self {
            Buffer::Command {
                scopes,
                depth,
                trailing,
                ..
            } => {
                *depth = 1;
                scopes.push((String::new(), variant, String::new()));
                if !trailing.is_empty() {
                    scopes.last_mut().unwrap().2 = std::mem::take(trailing);
                }
            }
            _ => unreachable!("pushing scope to non command element"),
        }
    }
}

impl FromStr for Document {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = Vec::new();

        // the current line number
        let mut line_no: u32 = 1;

        // whether the next character should be escaped
        let mut escaped = false;

        // whether the current character is a comment
        let mut comment = false;

        // line which the current buffer starts in
        let mut buffer_line_no = line_no;
        let mut buffer = Buffer::Text {
            content: String::new(),
        };

        // flush buffer content into chunks
        fn flush(
            buffer: &mut Buffer,
            buffer_line_no: u32,
            chunks: &mut Vec<Chunk>,
        ) -> Result<(), crate::Error> {
            macro_rules! eval_scope {
                ($i:ident) => {
                    Document::from_str($i)?
                        .chunks_owned()
                };
            }

            match buffer {
                Buffer::Text { ref mut content } if !content.is_empty() => chunks.push(Chunk::new(
                    buffer_line_no,
                    ChunkVariant::Text(std::mem::take(content)),
                )),
                Buffer::Text { .. } => {}
                Buffer::Scope { depth, variant, .. } if *depth != 0 => {
                    return Err(crate::Error::new(
                        1,
                        crate::ErrorType::UnclosedScope(*variant),
                    ))
                }
                Buffer::Scope { content, .. } => chunks.push(Chunk::new(
                    buffer_line_no,
                    ChunkVariant::Scope(Scope::new(eval_scope!(content), ScopeVariant::Curly)),
                )),
                Buffer::Command { depth, scopes, .. } if *depth != 0 => {
                    return Err(crate::Error::new(
                        1,
                        crate::ErrorType::UnclosedArgument(scopes.last().unwrap().1),
                    ))
                }
                Buffer::Command { label, scopes, .. } => {
                    let mut arguments = Vec::with_capacity(scopes.len());

                    for (content, variant, preceding) in scopes.iter_mut() {
                        arguments.push((
                            std::mem::take(preceding),
                            Scope::new(eval_scope!(content), *variant),
                        ))
                    }

                    chunks.push(Chunk::new(
                        buffer_line_no,
                        ChunkVariant::Command(Command::new(std::mem::take(label), arguments)),
                    ))
                }
            }

            Ok(())
        }

        // map flush errors lines to its absolute line number
        macro_rules! flush {
            () => {
                flush(&mut buffer, buffer_line_no, &mut chunks).map_err(|mut e| {
                    e.line += buffer_line_no - 1;
                    e
                })?;

                buffer_line_no = line_no;

                let _ = buffer_line_no; // theres a really annoying warning, this removes it
            };
        }

        for c in s.chars() {
            match c {
                '\n' => {
                    comment = false;
                    line_no += 1
                }
                '%' if !escaped => {
                    comment = true;
                    continue;
                }
                '\\' if !escaped => {
                    escaped = true;
                    continue;
                }
                _ if comment => continue,
                _ => {}
            }

            match &mut buffer {
                Buffer::Text { .. } => match c {
                    c if escaped => {
                        flush!();
                        buffer = Buffer::command();
                        buffer.push(c);

                        if !c.is_ascii_alphabetic() {
                            flush!();
                            buffer = Buffer::text();
                        }
                    }
                    c if ScopeVariant::is_opening(c) => {
                        flush!();
                        buffer = Buffer::scope(ScopeVariant::from_opening(c))
                    }
                    c => buffer.push(c),
                },
                Buffer::Scope { depth, variant, .. } => match c {
                    c if *depth != 0 && escaped => {
                        buffer.push('\\');
                        buffer.push(c);
                    }
                    c if variant.open() == c => {
                        *depth += 1;
                        buffer.push(c)
                    }
                    c if variant.close() == c => {
                        *depth -= 1;
                        if *depth == 0 {
                            flush!();
                            buffer = Buffer::text()
                        } else {
                            buffer.push(c)
                        }
                    }
                    c => buffer.push(c),
                },
                Buffer::Command {
                    depth,
                    scopes,
                    trailing,
                    ..
                } => match c {
                    c if *depth == 0 && ScopeVariant::is_opening(c) => {
                        buffer.push_scope(ScopeVariant::from_opening(c))
                    }
                    c if *depth == 0 && c.is_ascii_whitespace() => trailing.push(c),
                    c if *depth == 0 && escaped => {
                        let trailing = trailing.clone();
                        flush!();

                        if !trailing.is_empty() {
                            buffer = Buffer::text();
                            buffer.push_str(&trailing);

                            flush(
                                &mut buffer,
                                buffer_line_no
                                    - trailing.chars().filter(|c| *c == '\n').count() as u32,
                                &mut chunks,
                            )?;
                        }

                        buffer_line_no = line_no;

                        buffer = Buffer::command();
                        buffer.push(c);
                        if !c.is_ascii_alphabetic() {
                            flush!();
                            buffer = Buffer::text();
                        }
                    }
                    c if *depth != 0 && escaped => {
                        buffer.push('\\');
                        buffer.push(c);
                    }
                    c if *depth == 0 && !trailing.is_empty() => {
                        let trailing = trailing.clone();
                        flush!();
                        buffer = Buffer::text();
                        buffer.push_str(&trailing);
                        buffer.push(c);

                        buffer_line_no =
                            line_no - trailing.chars().filter(|c| *c == '\n').count() as u32;
                    }
                    c if *depth != 0 && scopes.last().unwrap().1.open() == c => *depth += 1,
                    c if *depth != 0 && scopes.last().unwrap().1.close() == c => *depth -= 1,
                    c if *depth == 0 && ScopeVariant::is_closing(c) => {
                        return Err(crate::Error::new(
                            line_no,
                            crate::ErrorType::UnexpectedClosing(ScopeVariant::from_closing(c)),
                        ))
                    }
                    c if *depth == 0 && !buffer.no_scope() => {
                        flush!();
                        buffer = Buffer::text();
                        buffer.push(c);
                    }
                    c => buffer.push(c),
                },
            }

            escaped = false;
        }

        match &buffer {
            // if its a command at the end of document,
            // there may be unflushed Text in the trailing buffer
            // so create an extra chunk if there are content in the trailing buffer
            Buffer::Command { trailing, .. } if !trailing.is_empty() => {
                let trailing = trailing.clone();
                flush!();
                buffer = Buffer::text();
                buffer.push_str(&trailing);
                flush!();
            }
            _ => {
                flush!();
            }
        }

        Ok(Self(chunks))
    }
}
