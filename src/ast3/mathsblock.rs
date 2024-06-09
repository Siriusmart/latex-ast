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
            DoubleClosing(MathsVariant),
            None,
        }

        let mut out = Vec::new();
        let mut mode = MathsMode::None;
        let mut depth: u32 = 0;

        let mut buffer = Vec::new();
        let mut buffer_line = 0;

        for chunk in chunks {
            let (line_no, variant) = chunk.decompose();

            macro_rules! push {
                ($x:expr) => {{
                    if let MathsMode::DoubleClosing(var) = mode {
                        mode = MathsMode::DoubleDollar(var)
                    }

                    out.push(Chunk::new(line_no, $x))
                }};
            }

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
                            MathsType::Outline,
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
                            MathsType::Inline,
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
                    push!(ChunkVariant::Scope(s.try_into()?))
                }
                ast2::ChunkVariant::Environment(env) if mode == MathsMode::None => {
                    push!(ChunkVariant::Environment(env.try_into()?))
                }
                ast2::ChunkVariant::Command(cmd) if mode == MathsMode::None => {
                    push!(ChunkVariant::Command(cmd.try_into()?))
                }
                ast2::ChunkVariant::Text(s) if mode == MathsMode::None => {
                    push!(ChunkVariant::Text(s))
                }
                ast2::ChunkVariant::Text(_) => todo!(),
                _ => buffer.push(ast2::Chunk::new(line_no, variant)),
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
