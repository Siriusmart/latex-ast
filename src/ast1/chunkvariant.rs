use std::fmt::Display;

use crate::traits::{Lines, Validate};

use super::{command::Command, scope::Scope, ScopeVariant};

/// Different types of things a chunk can be
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub enum ChunkVariant {
    /// Basic block of string
    Text(String),
    /// A single command and its following arguments
    Command(Command),
    /// A single scope
    Scope(Scope),
}

impl Display for ChunkVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => f.write_str(s),
            Self::Scope(scope) => f.write_fmt(format_args!("{scope}")),
            Self::Command(command) => f.write_fmt(format_args!("{command}")),
        }
    }
}

impl Validate for ChunkVariant {
    fn validate(&self) -> Result<(), crate::InternalError> {
        match self {
            Self::Text(s) => {
                for c in s.chars() {
                    if matches!(c, '\\' | '%')
                        || ScopeVariant::is_opening(c)
                        || ScopeVariant::is_closing(c)
                    {
                        return Err(crate::InternalError::UnsanitisedCharInString(c));
                    }
                }
            }
            Self::Command(c) => return c.validate(),
            Self::Scope(sc) => return sc.validate(),
        }

        Ok(())
    }
}

impl Lines for ChunkVariant {
    fn lines(&self) -> u32 {
        match self {
            Self::Text(s) => s.chars().filter(|c| c == &'\n').count() as u32 + 1,
            Self::Command(c) => c.lines(),
            Self::Scope(sc) => sc.lines(),
        }
    }
}
