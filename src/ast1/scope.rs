use std::fmt::Display;

use crate::{
    ast2,
    traits::{Lines, Validate},
    InternalError,
};

use super::{chunk::Chunk, scopevariant::ScopeVariant, IntoChunks};

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

impl Validate for Scope {
    fn validate(&self) -> Result<(), crate::InternalError> {
        for chunk in self.chunks.iter() {
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

impl From<ast2::Scope> for Scope {
    fn from(value: ast2::Scope) -> Self {
        let (chunks, variant) = value.decompose();
        Scope::new_unchecked(
            chunks
                .into_iter()
                .flat_map(ast2::Chunk::into_chunks)
                .collect(),
            variant.into(),
        )
    }
}
