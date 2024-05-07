use std::fmt::Display;

use super::{command::Command, scope::Scope};

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
