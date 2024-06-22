use std::fmt::Display;

use crate::{
    ast1::{self, IntoChunks},
    traits::{Lines, Validate},
    InternalError,
};

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

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.clone().to_ast1_scope()))
    }
}

impl Scope {
    /// Maps to `ast1::Scope`
    pub fn to_ast1_scope(self) -> ast1::Scope {
        ast1::Scope::new_unchecked(
            self.chunks
                .into_iter()
                .flat_map(Chunk::into_chunks)
                .collect(),
            self.variant.into(),
        )
    }
}

impl Validate for Scope {
    fn validate(&self) -> Result<(), crate::InternalError> {
        for chunk in self.chunks() {
            chunk.validate()?
        }

        Ok(())
    }
}

impl Lines for Scope {
    fn lines(&self) -> u32 {
        self.chunks()
            .iter()
            .map(|chunk| chunk.lines() - 1)
            .sum::<u32>()
            + 1
    }
}

impl Scope {
    /// Create new scope from its content and the scope variant
    pub fn new(chunks: Vec<Chunk>, variant: ScopeVariant) -> Result<Self, InternalError> {
        let out = Self { chunks, variant };
        out.validate()?;
        Ok(out)
    }

    /// Create new scope from its content and the scope variant without checking
    pub fn new_unchecked(chunks: Vec<Chunk>, variant: ScopeVariant) -> Self {
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
            chunks: Document::try_from(ast1::Document::new_unchecked(value.chunks_owned()))?
                .chunks_owned(),
        })
    }
}
