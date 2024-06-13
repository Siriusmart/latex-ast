use super::{Command, Environment, MathsBlock, Scope};

/// Different types of things a chunk can be
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
    /// Whitespaces between paragraphs
    InterParagraph(String),
    /// A single command and its following arguments
    Command(Command),
    /// A single scope
    Scope(Scope),
    /// An environmentnis a labelled scope with options
    Environment(Environment),
}
