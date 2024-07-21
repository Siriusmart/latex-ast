use crate::ast3;

use super::{Command, Environment, MathsBlock, Scope};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub enum ChunkVariant {
    /// A block of text
    Text(String),
    /// Paragraph breaking
    ParagraphBreak(String),
    /// A block of inline or outline maths
    MathsBlock(MathsBlock),
    /// A single command and its following arguments
    Command(Command),
    /// A single scope
    Scope(Scope),
    /// An environmentnis a labelled scope with options
    Environment(Environment),
}

impl From<ast3::ChunkVariant> for ChunkVariant {
    fn from(value: ast3::ChunkVariant) -> Self {
        todo!()
    }
}
