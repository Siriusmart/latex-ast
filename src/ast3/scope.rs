use crate::{
    ast2,
    traits::{Lines, Validate},
    InternalError,
};

use super::{Chunk, MathsBlock, Paragraph, ScopeVariant};

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

impl TryFrom<ast2::Scope> for Scope {
    type Error = crate::Error;

    fn try_from(value: ast2::Scope) -> Result<Self, Self::Error> {
        let (chunks, variant) = value.decompose();

        Ok(Self::new_unchecked(
            Paragraph::from_chunks(MathsBlock::from_chunks(chunks)?),
            variant.into(),
        ))
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
