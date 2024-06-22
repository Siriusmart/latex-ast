use crate::{
    ast2,
    ast3::{ChunkVariant, MathsVariant},
};

use super::{Chunk, MathsType};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct MathsBlock {
    variant: MathsVariant,
    r#type: MathsType,
    content: Vec<Chunk>,
}

impl MathsBlock {
    pub fn new(variant: MathsVariant, r#type: MathsType, content: Vec<Chunk>) -> Self {
        Self {
            variant,
            r#type,
            content,
        }
    }
}

impl MathsBlock {
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
                    out.push(Chunk::new(
                        buffer_line,
                        ChunkVariant::MathsBlock(Self::new(
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
                    out.push(Chunk::new(
                        buffer_line,
                        ChunkVariant::MathsBlock(Self::new(
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
                    out.push(Chunk::new(line_no, ChunkVariant::Scope(s.try_into()?)));
                }
                ast2::ChunkVariant::Environment(env) if mode == MathsMode::None => {
                    out.push(Chunk::new(
                        line_no,
                        ChunkVariant::Environment(env.try_into()?),
                    ));
                }
                ast2::ChunkVariant::Command(cmd) if mode == MathsMode::None => {
                    out.push(Chunk::new(line_no, ChunkVariant::Command(cmd.try_into()?)));
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
                    let mut text_buffer_line = 1;

                    macro_rules! push_str {
                        () => {
                            if !text_buffer.is_empty() {
                                out.push(Chunk::new(
                                    text_buffer_line + line_no - 1,
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

                                    out.push(Chunk::new(
                                        text_buffer_line,
                                        ChunkVariant::MathsBlock(MathsBlock::new(
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
                                    ))
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

                                    out.push(Chunk::new(
                                        text_buffer_line,
                                        ChunkVariant::MathsBlock(MathsBlock::new(
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
                                    ))
                                }
                                MathsMode::SingleDollar(MathsVariant::Brackets)
                                | MathsMode::DoubleDollar(MathsVariant::Brackets) => {}
                            }
                        } else {
                            text_buffer.push(c)
                        }
                    }

                    if !text_buffer.is_empty() {
                        if mode == MathsMode::None {
                            out.push(Chunk::new(
                                text_buffer_line + line_no - 1,
                                ChunkVariant::Text(text_buffer),
                            ))
                        } else {
                            buffer.push(ast2::Chunk::new_unchecked(
                                text_buffer_line + line_no - 1,
                                ast2::ChunkVariant::Text(text_buffer),
                            ))
                        }
                    }
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

        Ok(out)
    }
}
