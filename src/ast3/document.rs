use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::{
    ast2,
    ast3::{Environment, MathsBlock, Paragraph},
    traits::{Lines, Validate},
    InternalError,
};

use super::{Chunk, ChunkVariant, Command, Scope};

/// Main struct for stage 3 AST
///
/// Display `{}` reconstructs the original document
#[derive(Default, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Document {
    documentclass: Option<String>,
    // prec, key, (equal prec, equal post, val), post
    documentoptions: Vec<(String, String, Option<(String, String, String)>, String)>,

    preamable: Vec<Chunk>,

    body: Vec<Chunk>,
    body_args: Vec<(String, Scope)>,
    body_begin_prec: String,
    body_end_prec: String,

    trailing: Vec<Chunk>,
}

impl Validate for Document {
    fn validate(&self) -> Result<(), crate::InternalError> {
        for section in [&self.preamable, &self.body, &self.trailing] {
            let mut expected_line = 1;

            for chunk in section.iter() {
                if chunk.line_no() != expected_line {
                    return Err(InternalError::IncorrectChunkLineNumber {
                        expected: expected_line,
                        got: chunk.line_no(),
                    });
                }

                chunk.validate()?;

                expected_line += chunk.lines() - 1;
            }
        }

        Ok(())
    }
}

impl Lines for Document {
    fn lines(&self) -> u32 {
        self.preamable
            .iter()
            .chain(self.body.iter())
            .chain(self.trailing.iter())
            .map(|chunk| chunk.lines() - 1)
            .sum::<u32>()
            + 1
    }
}

impl Document {
    /// Create new document
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        preamable: Vec<Chunk>,
        documentclass: Option<String>,
        documentoptions: Vec<(String, String, Option<(String, String, String)>, String)>,
        body: Vec<Chunk>,
        body_args: Vec<(String, Scope)>,
        body_begin_prec: String,
        body_end_prec: String,
        trailing: Vec<Chunk>,
    ) -> Result<Self, InternalError> {
        let out = Self::new_unchecked(
            preamable,
            documentclass,
            documentoptions,
            body,
            body_args,
            body_begin_prec,
            body_end_prec,
            trailing,
        );
        out.validate()?;
        Ok(out)
    }

    /// Create new document without checking
    #[allow(clippy::too_many_arguments)]
    pub fn new_unchecked(
        preamable: Vec<Chunk>,
        documentclass: Option<String>,
        documentoptions: Vec<(String, String, Option<(String, String, String)>, String)>,
        body: Vec<Chunk>,
        body_args: Vec<(String, Scope)>,
        body_begin_prec: String,
        body_end_prec: String,
        trailing: Vec<Chunk>,
    ) -> Self {
        Self {
            documentclass,
            documentoptions,
            preamable,
            body,
            body_args,
            body_begin_prec,
            body_end_prec,
            trailing,
        }
    }

    /// Returns the chunks representing the preamable
    pub fn chunks_preamable(&self) -> &Vec<Chunk> {
        &self.preamable
    }

    /// Returns the chunks representing the body
    pub fn chunks_body(&self) -> &Vec<Chunk> {
        &self.body
    }

    /// Returns the chunks representing the trailing
    pub fn chunks_trailing(&self) -> &Vec<Chunk> {
        &self.trailing
    }

    /// Push a Chunk to a `Vec<Chunk>` without checking
    fn push_chunk_vec_unchecked(vec: &mut Vec<Chunk>, chunk: Chunk) {
        if let ChunkVariant::Text(s) = chunk.variant() {
            if let Some(last) = vec.last_mut() {
                if let ChunkVariant::Text(last_s) = last.variant_mut() {
                    last_s.push_str(s);
                    return;
                }
            }
        }

        vec.push(chunk)
    }

    /// Push a Chunk to a `Vec<Chunk>`
    fn push_chunk_vec(vec: &mut Vec<Chunk>, chunk: Chunk) -> Result<(), InternalError> {
        chunk.validate()?;

        let expected_line = vec
            .last()
            .map(|last| last.line_no() + last.lines() - 1)
            .unwrap_or(1);

        if expected_line != chunk.line_no() {
            return Err(crate::InternalError::IncorrectChunkLineNumber {
                expected: expected_line,
                got: chunk.line_no(),
            });
        }

        Self::push_chunk_vec_unchecked(vec, chunk);
        Ok(())
    }

    /// Push a ChunkVariant to a `Vec<Chunk>` without checking
    fn push_vec_unchecked(vec: &mut Vec<Chunk>, variant: ChunkVariant) {
        let line_no = vec
            .last()
            .map(|last| last.line_no() + last.lines() - 1)
            .unwrap_or(1);
        Self::push_chunk_vec_unchecked(vec, Chunk::new_unchecked(line_no, variant));
    }

    /// Push a ChunkVariant to a `Vec<Chunk>`
    fn push_vec(vec: &mut Vec<Chunk>, variant: ChunkVariant) -> Result<(), InternalError> {
        variant.validate()?;
        Self::push_vec_unchecked(vec, variant);
        Ok(())
    }

    /// Push a ChunkVarint to body
    pub fn push_body(&mut self, variant: ChunkVariant) -> Result<(), InternalError> {
        Self::push_vec(&mut self.body, variant)
    }

    /// Push a ChunkVarint to body without checking
    pub fn push_body_unchecked(&mut self, variant: ChunkVariant) {
        Self::push_vec_unchecked(&mut self.body, variant)
    }

    /// Push a Chunk to body
    pub fn push_body_chunk(&mut self, chunk: Chunk) -> Result<(), InternalError> {
        Self::push_chunk_vec(&mut self.body, chunk)
    }

    /// Push a Chunk to body without checking
    pub fn push_body_chunk_unchecked(&mut self, chunk: Chunk) {
        Self::push_chunk_vec_unchecked(&mut self.body, chunk)
    }

    /// Push a ChunkVariant to preamable
    pub fn push_preamable(&mut self, variant: ChunkVariant) -> Result<(), InternalError> {
        Self::push_vec(&mut self.preamable, variant)
    }

    /// Push a ChunkVariant to preamable without checking
    pub fn push_preamable_unchecked(&mut self, variant: ChunkVariant) {
        Self::push_vec_unchecked(&mut self.preamable, variant)
    }

    /// Push a Chunk to preamable
    pub fn push_preamable_chunk(&mut self, chunk: Chunk) -> Result<(), InternalError> {
        Self::push_chunk_vec(&mut self.preamable, chunk)
    }

    /// Push a Chunk to preamable without checking
    pub fn push_preamable_chunk_unchecked(&mut self, chunk: Chunk) {
        Self::push_chunk_vec_unchecked(&mut self.preamable, chunk)
    }

    /// Push a ChunkVarint to trailing
    pub fn push_trailing(&mut self, variant: ChunkVariant) -> Result<(), InternalError> {
        Self::push_vec(&mut self.trailing, variant)
    }

    /// Push a ChunkVariant to trailing without checking
    pub fn push_trailing_unchecked(&mut self, variant: ChunkVariant) {
        Self::push_vec_unchecked(&mut self.trailing, variant)
    }

    /// Push a Chunk to trailing
    pub fn push_trailing_chunk(&mut self, chunk: Chunk) -> Result<(), InternalError> {
        Self::push_chunk_vec(&mut self.trailing, chunk)
    }

    /// Push a Chunk to trailing without checking
    pub fn push_trailing_chunk_unchecked(&mut self, chunk: Chunk) {
        Self::push_chunk_vec_unchecked(&mut self.trailing, chunk)
    }

    /// Returns all fields of this struct
    #[allow(clippy::type_complexity)]
    pub fn decompose(
        self,
    ) -> (
        Option<String>,
        Vec<(String, String, Option<(String, String, String)>, String)>,
        Vec<Chunk>,
        Vec<Chunk>,
        Vec<(String, Scope)>,
        String,
        String,
        Vec<Chunk>,
    ) {
        (
            self.documentclass,
            self.documentoptions,
            self.preamable,
            self.body,
            self.body_args,
            self.body_begin_prec,
            self.body_end_prec,
            self.trailing,
        )
    }
}

impl TryFrom<ast2::Document> for Document {
    type Error = crate::Error;

    fn try_from(value: ast2::Document) -> Result<Self, Self::Error> {
        let mut construct = Self::default();

        let mut preamable = Vec::new();
        let mut trailing = Vec::new();

        #[derive(PartialEq)]
        enum CursorState {
            Preamable,
            Trailing,
        }

        let mut cursor = CursorState::Preamable;

        for chunk in value.chunks_owned() {
            let (line_no, variant) = chunk.decompose();

            match variant {
                ast2::ChunkVariant::Environment(env) if env.label().as_str() == "document" => {
                    let (_, args, content, begin, end) = Environment::try_from(env)?.decompose();
                    construct.body_args = args;
                    construct.body = content;
                    construct.body_begin_prec = begin;
                    construct.body_end_prec = end;
                    cursor = CursorState::Trailing;
                }
                ast2::ChunkVariant::Command(ref cmd)
                    if cmd.label().as_str() == "documentclass"
                        && cursor == CursorState::Preamable =>
                {
                    if construct.documentclass.is_some() {
                        return Err(crate::Error::new(
                            line_no,
                            crate::ErrorType::DoubleDocumentClass,
                        ));
                    }
                    for (_, scope) in cmd.clone().decompose().1 {
                        match scope.variant() {
                            ast2::ScopeVariant::Curly if construct.documentclass.is_some() => {
                                return Err(crate::Error::new(
                                    line_no,
                                    crate::ErrorType::TooManyArgsDocumentClass,
                                ))
                            }
                            ast2::ScopeVariant::Curly => {
                                construct.documentclass = Some(
                                    ast2::Document::new_unchecked(scope.chunks_owned()).to_string(),
                                )
                            }
                            _ => {
                                for opt in ast2::Document::new_unchecked(scope.chunks_owned())
                                    .to_string()
                                    .split(',')
                                {
                                    let mut prec = String::new();
                                    let mut arg = String::new();
                                    let mut post = String::new();

                                    enum BufferMode {
                                        Prec,
                                        Arg,
                                        PotentialPost,
                                    }

                                    let mut buffer = String::new();
                                    let mut buffermode = BufferMode::Prec;

                                    for c in opt.chars() {
                                        match buffermode {
                                            BufferMode::Prec if c.is_whitespace() => buffer.push(c),
                                            BufferMode::Prec => {
                                                prec.push_str(&buffer);
                                                buffer.clear();
                                                buffermode = BufferMode::Arg;
                                            }
                                            BufferMode::Arg if !c.is_whitespace() => arg.push(c),
                                            BufferMode::Arg => {
                                                buffer.push(c);
                                                buffermode = BufferMode::PotentialPost;
                                            }
                                            BufferMode::PotentialPost if c.is_whitespace() => {
                                                buffer.push(c)
                                            }
                                            BufferMode::PotentialPost => {
                                                arg.push_str(&buffer);
                                                arg.push(c);
                                                buffer.clear();
                                                buffermode = BufferMode::Arg;
                                            }
                                        }
                                    }

                                    if !buffer.is_empty() {
                                        post = buffer;
                                    }

                                    if let Some((k, v)) = arg.split_once('=') {
                                        let mut key = String::new();
                                        let mut val = String::new();
                                        let mut equal_prec = String::new();
                                        let mut equal_post = String::new();

                                        {
                                            let mut buffer = String::new();

                                            for c in k.chars() {
                                                if c.is_whitespace() {
                                                    buffer.push(c)
                                                } else {
                                                    if !buffer.is_empty() {
                                                        key.push_str(&buffer);
                                                        buffer.clear();
                                                    }
                                                    key.push(c)
                                                }
                                            }

                                            if !buffer.is_empty() {
                                                equal_prec = buffer;
                                            }
                                        }

                                        {
                                            let mut val_started = false;

                                            for c in k.chars() {
                                                if c.is_whitespace() && !val_started {
                                                    equal_post.push(c)
                                                } else {
                                                    if val_started {
                                                        val_started = true;
                                                    }

                                                    val.push(c)
                                                }
                                            }
                                        }
                                        construct.documentoptions.push((
                                            prec,
                                            key,
                                            Some((equal_prec, equal_post, val)),
                                            post,
                                        ));
                                    } else {
                                        construct.documentoptions.push((prec, arg, None, post));
                                    }
                                }
                            }
                        }
                    }
                    preamable.push(ast2::Chunk::new_unchecked(line_no, variant))
                }
                _ if cursor == CursorState::Preamable => {
                    preamable.push(ast2::Chunk::new_unchecked(line_no, variant))
                }
                _ => trailing.push(ast2::Chunk::new_unchecked(line_no, variant)),
            }
        }

        construct.preamable = Paragraph::from_chunks(MathsBlock::from_chunks(preamable)?);
        construct.trailing = Paragraph::from_chunks(MathsBlock::from_chunks(trailing)?);

        Ok(construct)
    }
}

impl FromStr for Document {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ast2::Document::from_str(s)?.try_into()
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.preamable
                .iter()
                .chain(
                    [Chunk::new_unchecked(
                        1,
                        ChunkVariant::Command(Command::new_unchecked(
                            "begin".to_string(),
                            [(
                                self.body_begin_prec.clone(),
                                Scope::new_unchecked(
                                    vec![Chunk::new_unchecked(
                                        1,
                                        ChunkVariant::Text("document".to_string()),
                                    )],
                                    super::ScopeVariant::Curly,
                                ),
                            )]
                            .into_iter()
                            .chain(self.body_args.clone())
                            .collect(),
                        )),
                    )]
                    .iter(),
                )
                .chain(self.body.iter())
                .chain(
                    [Chunk::new_unchecked(
                        1,
                        ChunkVariant::Command(Command::new_unchecked(
                            "end".to_string(),
                            vec![(
                                self.body_begin_prec.clone(),
                                Scope::new_unchecked(
                                    vec![Chunk::new_unchecked(
                                        1,
                                        ChunkVariant::Text("document".to_string()),
                                    )],
                                    super::ScopeVariant::Curly,
                                ),
                            )],
                        )),
                    )]
                    .iter(),
                )
                .chain(self.trailing.iter())
                .map(ToString::to_string)
                .collect::<String>()
                .as_str(),
        )
    }
}
