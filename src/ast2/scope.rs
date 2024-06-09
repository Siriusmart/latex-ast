use crate::ast1::{self, IntoChunks};

use super::{Chunk, Document, ScopeVariant};

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
    /// Maps to `ast1::Scope`
    pub fn to_ast1_scope(self) -> ast1::Scope {
        ast1::Scope::new(
            self.chunks
                .into_iter()
                .flat_map(Chunk::into_chunks)
                .collect(),
            self.variant.into(),
        )
    }
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

    /// Returns all fields of this struct
    pub fn decompose(self) -> (Vec<Chunk>, ScopeVariant) {
        (self.chunks, self.variant)
    }
}

impl TryFrom<crate::ast1::Scope> for Scope {
    type Error = crate::Error;

    fn try_from(value: crate::ast1::Scope) -> Result<Self, Self::Error> {
        Ok(Self {
            variant: value.variant().into(),
            chunks: Document::try_from(ast1::Document::new(value.chunks_owned()))?.chunks_owned(),
        })
    }
}
