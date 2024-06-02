use std::fmt::Display;

use super::{chunk::Chunk, scopevariant::ScopeVariant};

/// A scoped block
///
/// Note that is cannot exist independently immediately following a command without
/// any nonwhitespace character in between
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Scope {
    chunks: Vec<Chunk>,
    variant: ScopeVariant,
}

impl Scope {
    /// Create new scope from its content and the scope variant
    pub fn new(chunks: Vec<Chunk>, variant: ScopeVariant) -> Self {
        Self { chunks, variant }
    }

    /// Returns all chunks within
    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }

    /// Returns all owned chunks within
    pub fn chunks_owned(self) -> Vec<Chunk> {
        self.chunks
    }

    /// Returns the scope variant
    pub fn variant(&self) -> ScopeVariant {
        self.variant
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
