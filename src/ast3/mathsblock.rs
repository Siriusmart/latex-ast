use std::fmt::Display;

use crate::{
    ast2::{self, IntoChunks},
    ast3::{ChunkVariant, MathsVariant, Paragraph},
    traits::{Lines, Validate},
    InternalError,
};

use super::{Chunk, MathsType};

/// A block of maths environment, surrounded by $, $$, \[ or \(
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct MathsBlock {
    variant: MathsVariant,
    r#type: MathsType,
    content: Vec<Chunk>,
}

impl MathsBlock {
    /// Construct a new MathsBlock
    pub fn new(
        variant: MathsVariant,
        r#type: MathsType,
        content: Vec<Chunk>,
    ) -> Result<Self, InternalError> {
        let out = Self::new_unchecked(variant, r#type, content);
        out.validate()?;
        Ok(out)
    }

    /// Construct a new MathsBlock without checking
    pub fn new_unchecked(variant: MathsVariant, r#type: MathsType, content: Vec<Chunk>) -> Self {
        Self {
            variant,
            r#type,
            content,
        }
    }
}

impl Validate for MathsBlock {
    fn validate(&self) -> Result<(), crate::InternalError> {
        for chunk in self.content.iter() {
            chunk.validate()?
        }

        Ok(())
    }
}

impl Lines for MathsBlock {
    fn lines(&self) -> u32 {
        self.content
            .iter()
            .map(|chunk| chunk.lines() - 1)
            .sum::<u32>()
            + 1
    }
}

impl MathsBlock {
    /// Maps a `Vec<ast2::Chunk>` to `Vec<Chunk>` with MathsBlocks
    pub fn from_chunks(chunks: Vec<ast2::Chunk>) -> Result<Vec<Chunk>, crate::Error> {
        #[derive(PartialEq)]
        enum MathsMode {
            SingleDollar(MathsVariant),
            DoubleDollar(MathsVariant),
            DoubleClosing,
            None,
        }

        let mut out = Vec::new();
        let mut mode = MathsMode::None;
        let mut depth: u32 = 0;

        let mut buffer = Vec::new();
        let mut buffer_line = 0;

        for chunk in chunks {
            let (line_no, variant) = chunk.decompose();

            match variant {
                ast2::ChunkVariant::Command(cmd)
                    if mode == MathsMode::None && matches!(cmd.label().as_str(), ")" | "]") =>
                {
                    return Err(crate::Error::new(
                        line_no,
                        crate::ErrorType::UnexpectedMathsEnd,
                    ))
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == "(" && mode == MathsMode::None =>
                {
                    mode = MathsMode::SingleDollar(MathsVariant::Brackets);
                    buffer_line = line_no;
                    depth = 1;
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == "[" && mode == MathsMode::None =>
                {
                    mode = MathsMode::DoubleDollar(MathsVariant::Brackets);
                    buffer_line = line_no;
                    depth = 1;
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == "("
                        && mode == MathsMode::SingleDollar(MathsVariant::Brackets) =>
                {
                    depth += 1
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == "["
                        && mode == MathsMode::DoubleDollar(MathsVariant::Brackets) =>
                {
                    depth += 1
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == ")"
                        && mode == MathsMode::SingleDollar(MathsVariant::Brackets)
                        && depth == 1 =>
                {
                    mode = MathsMode::None;
                    depth = 0;
                    out.push(Chunk::new_unchecked(
                        buffer_line,
                        ChunkVariant::MathsBlock(Self::new_unchecked(
                            MathsVariant::Brackets,
                            MathsType::Inline,
                            Self::from_chunks(std::mem::take(&mut buffer))?,
                        )),
                    ))
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == "]"
                        && mode == MathsMode::DoubleDollar(MathsVariant::Brackets)
                        && depth == 1 =>
                {
                    mode = MathsMode::None;
                    depth = 0;
                    out.push(Chunk::new_unchecked(
                        buffer_line,
                        ChunkVariant::MathsBlock(Self::new_unchecked(
                            MathsVariant::Brackets,
                            MathsType::Outline,
                            Self::from_chunks(std::mem::take(&mut buffer))?,
                        )),
                    ))
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == ")"
                        && mode == MathsMode::SingleDollar(MathsVariant::Brackets) =>
                {
                    depth -= 1
                }
                ast2::ChunkVariant::Command(cmd)
                    if cmd.label() == "]"
                        && mode == MathsMode::DoubleDollar(MathsVariant::Brackets) =>
                {
                    depth -= 1
                }
                ast2::ChunkVariant::Scope(s) if mode == MathsMode::None => {
                    out.push(Chunk::new_unchecked(
                        line_no,
                        ChunkVariant::Scope(s.try_into()?),
                    ));
                }
                ast2::ChunkVariant::Environment(env) if mode == MathsMode::None => {
                    out.push(Chunk::new_unchecked(
                        line_no,
                        ChunkVariant::Environment(env.try_into()?),
                    ));
                }
                ast2::ChunkVariant::Command(cmd) if mode == MathsMode::None => {
                    out.push(Chunk::new_unchecked(
                        line_no,
                        ChunkVariant::Command(cmd.try_into()?),
                    ));
                }
                ast2::ChunkVariant::Text(s)
                    if !matches!(
                        mode,
                        MathsMode::SingleDollar(MathsVariant::Brackets)
                            | MathsMode::DoubleDollar(MathsVariant::Brackets)
                    ) =>
                {
                    let mut cursor_line_no = line_no;
                    let mut text_buffer = String::new();
                    let mut text_buffer_line = cursor_line_no;

                    macro_rules! push_str {
                        () => {
                            if !text_buffer.is_empty() {
                                out.push(Chunk::new_unchecked(
                                    text_buffer_line,
                                    ChunkVariant::Text(std::mem::take(&mut text_buffer)),
                                ))
                            }
                        };
                    }

                    for c in s.chars() {
                        if c == '\n' {
                            cursor_line_no += 1
                        }

                        if c == '$' {
                            match mode {
                                MathsMode::None => {
                                    push_str!();
                                    mode = MathsMode::SingleDollar(MathsVariant::Dollars);
                                    text_buffer_line = cursor_line_no;
                                }
                                MathsMode::SingleDollar(MathsVariant::Dollars)
                                    if text_buffer.is_empty() && buffer.is_empty() =>
                                {
                                    mode = MathsMode::DoubleDollar(MathsVariant::Dollars)
                                }
                                MathsMode::DoubleDollar(MathsVariant::Dollars) => {
                                    mode = MathsMode::DoubleClosing
                                }
                                MathsMode::DoubleClosing => {
                                    mode = MathsMode::None;

                                    if !text_buffer.is_empty() {
                                        buffer.push(ast2::Chunk::new_unchecked(
                                            text_buffer_line,
                                            ast2::ChunkVariant::Text(std::mem::take(
                                                &mut text_buffer,
                                            )),
                                        ));
                                    }

                                    out.push(Chunk::new_unchecked(
                                        text_buffer_line,
                                        ChunkVariant::MathsBlock(MathsBlock::new_unchecked(
                                            MathsVariant::Dollars,
                                            MathsType::Outline,
                                            Self::from_chunks(
                                                std::mem::take(&mut buffer)
                                                    .into_iter()
                                                    .map(|mut chunk| {
                                                        *chunk.line_no_mut() -=
                                                            text_buffer_line - 1;
                                                        chunk
                                                    })
                                                    .collect(),
                                            )?,
                                        )),
                                    ));

                                    text_buffer_line = cursor_line_no;
                                }
                                MathsMode::SingleDollar(MathsVariant::Dollars) => {
                                    mode = MathsMode::None;

                                    if !text_buffer.is_empty() {
                                        buffer.push(ast2::Chunk::new_unchecked(
                                            text_buffer_line,
                                            ast2::ChunkVariant::Text(std::mem::take(
                                                &mut text_buffer,
                                            )),
                                        ));
                                    }

                                    out.push(Chunk::new_unchecked(
                                        text_buffer_line,
                                        ChunkVariant::MathsBlock(MathsBlock::new_unchecked(
                                            MathsVariant::Dollars,
                                            MathsType::Inline,
                                            Self::from_chunks(
                                                std::mem::take(&mut buffer)
                                                    .into_iter()
                                                    .map(|mut chunk| {
                                                        *chunk.line_no_mut() -=
                                                            text_buffer_line - 1;
                                                        chunk
                                                    })
                                                    .collect(),
                                            )?,
                                        )),
                                    ));

                                    text_buffer_line = cursor_line_no;
                                }
                                MathsMode::SingleDollar(MathsVariant::Brackets)
                                | MathsMode::DoubleDollar(MathsVariant::Brackets) => {}
                            }
                        } else {
                            text_buffer.push(c)
                        }
                    }

                    push_str!()
                }
                _ => buffer.push(ast2::Chunk::new_unchecked(line_no, variant)),
            }
        }

        if mode != MathsMode::None {
            return Err(crate::Error::new(
                buffer_line,
                crate::ErrorType::UnclosedMaths,
            ));
        }

        out = Paragraph::from_chunks(out);

        Ok(out)
    }
}

impl IntoChunks for MathsBlock {
    fn into_chunks(self) -> Vec<ast2::Chunk> {
        let lines = self.lines();

        match (self.variant, &self.r#type) {
            (MathsVariant::Brackets, MathsType::Inline) => {
                return [ast2::Chunk::new_unchecked(
                    1,
                    ast2::ChunkVariant::Command(ast2::Command::new_unchecked(
                        "(".to_string(),
                        Vec::new(),
                    )),
                )]
                .into_iter()
                .chain(self.content.into_iter().flat_map(Chunk::into_chunks))
                .chain([ast2::Chunk::new_unchecked(
                    lines,
                    ast2::ChunkVariant::Command(ast2::Command::new_unchecked(
                        ")".to_string(),
                        Vec::new(),
                    )),
                )])
                .collect()
            }
            (MathsVariant::Brackets, MathsType::Outline) => {
                return [ast2::Chunk::new_unchecked(
                    1,
                    ast2::ChunkVariant::Command(ast2::Command::new_unchecked(
                        "[".to_string(),
                        Vec::new(),
                    )),
                )]
                .into_iter()
                .chain(self.content.into_iter().flat_map(Chunk::into_chunks))
                .chain([ast2::Chunk::new_unchecked(
                    lines,
                    ast2::ChunkVariant::Command(ast2::Command::new_unchecked(
                        "]".to_string(),
                        Vec::new(),
                    )),
                )])
                .collect()
            }
            _ => {}
        }

        let mut out: Vec<ast2::Chunk> = self
            .content
            .into_iter()
            .flat_map(Chunk::into_chunks)
            .collect();

        let dollars = if matches!(self.r#type, MathsType::Inline) {
            "$"
        } else {
            "$$"
        };

        if out.is_empty() {
            return vec![ast2::Chunk::new_unchecked(
                1,
                ast2::ChunkVariant::Text(dollars.repeat(2)),
            )];
        }

        if let ast2::ChunkVariant::Text(s) = out.first_mut().unwrap().variant_mut() {
            *s = format!("{dollars}{s}")
        } else {
            out.insert(
                0,
                ast2::Chunk::new_unchecked(1, ast2::ChunkVariant::Text(dollars.to_string())),
            )
        }

        if let ast2::ChunkVariant::Text(s) = out.last_mut().unwrap().variant_mut() {
            *s = format!("{s}{dollars}")
        } else {
            out.push(ast2::Chunk::new_unchecked(
                lines,
                ast2::ChunkVariant::Text(dollars.to_string()),
            ))
        }

        out
    }
}

impl Display for MathsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (open, close) = match (&self.variant, &self.r#type) {
            (MathsVariant::Brackets, MathsType::Outline) => ("\\[", "\\]"),
            (MathsVariant::Brackets, MathsType::Inline) => ("\\(", "\\)"),
            (MathsVariant::Dollars, MathsType::Outline) => ("$$", "$$"),
            (MathsVariant::Dollars, MathsType::Inline) => ("$", "$"),
        };

        f.write_fmt(format_args!(
            "{open}{}{close}",
            self.content
                .iter()
                .map(ToString::to_string)
                .collect::<String>(),
        ))
    }
}
