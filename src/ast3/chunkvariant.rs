use crate::{
    ast1,
    traits::{Lines, Validate},
};

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
    /// A single command and its following arguments
    Command(Command),
    /// A single scope
    Scope(Scope),
    /// An environmentnis a labelled scope with options
    Environment(Environment),
}

impl Validate for ChunkVariant {
    fn validate(&self) -> Result<(), crate::InternalError> {
        match self {
            Self::Text(s) => {
                for c in s.chars() {
                    if matches!(c, '\\' | '%')
                        || ast1::ScopeVariant::is_opening(c)
                        || ast1::ScopeVariant::is_closing(c)
                    {
                        return Err(crate::InternalError::UnsanitisedCharInString(c));
                    }
                }
            }
            Self::ParagraphBreak(inter) => {
                let mut new_lines: u32 = 0;

                for c in inter.chars() {
                    if !c.is_whitespace() {
                        return Err(crate::InternalError::ParagraghBreakNonwhitespace);
                    }

                    if c == '\n' {
                        new_lines += 1;
                    }
                }

                if new_lines < 2 {
                    return Err(crate::InternalError::ParagraphBreakTooShort);
                }
            }
            Self::MathsBlock(m) => return m.validate(),
            Self::Command(c) => return c.validate(),
            Self::Scope(sc) => return sc.validate(),
            Self::Environment(e) => return e.validate(),
        }

        Ok(())
    }
}

impl Lines for ChunkVariant {
    fn lines(&self) -> u32 {
        match self {
            Self::Text(s) | Self::ParagraphBreak(s) => {
                s.chars().filter(|c| c == &'\n').count() as u32 + 1
            }
            Self::Command(c) => c.lines(),
            Self::Scope(sc) => sc.lines(),
            Self::Environment(e) => e.lines(),
            Self::MathsBlock(mb) => mb.lines(),
        }
    }
}
