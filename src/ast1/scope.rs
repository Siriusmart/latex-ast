use std::fmt::Display;

use super::{chunk::Chunk, scopevariant::ScopeVariant};

/// A scoped block
///
/// Note that is cannot exist independently immediately following a command without
/// any nonwhitespace character in between
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Scope {
    chunks: Vec<Chunk>,
    variant: ScopeVariant,
}

impl Scope {
    pub fn new(chunks: Vec<Chunk>, variant: ScopeVariant) -> Self {
        Self { chunks, variant }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}",
            self.variant.open(),
            &self
                .chunks
                .iter()
                .map(ToString::to_string)
                .collect::<String>(),
            self.variant.close()
        ))
    }
}
