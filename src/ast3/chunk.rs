use crate::{
    ast2::{self, IntoChunks},
    traits::{Lines, Validate},
    InternalError,
};

use super::ChunkVariant;

/// A chunk is a block of self contained content
///
/// - `Vec<Chunk>` makes a Document
/// - Each chunk has a line number, indicating the line number its starting character is in
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Chunk {
    line_no: u32,
    variant: ChunkVariant,
}

impl Chunk {
    /// Constructs new Chunk
    pub fn new(line_no: u32, variant: ChunkVariant) -> Result<Self, InternalError> {
        let out = Self { line_no, variant };
        out.validate()?;
        Ok(out)
    }

    /// Constructs new Chunk without checking
    pub fn new_unchecked(line_no: u32, variant: ChunkVariant) -> Self {
        Self { line_no, variant }
    }

    /// Returns the relative line number of current chunk
    pub fn line_no(&self) -> u32 {
        self.line_no
    }

    /// Returns the variant of current chunk
    pub fn variant(&self) -> &ChunkVariant {
        &self.variant
    }

    /// Returns the mutable variant of current chunk
    pub fn variant_mut(&mut self) -> &mut ChunkVariant {
        &mut self.variant
    }

    /// Returns the owned variant of current chunk
    pub fn variant_owned(self) -> ChunkVariant {
        self.variant
    }

    /// Returns all fields of this struct
    pub fn decompose(self) -> (u32, ChunkVariant) {
        (self.line_no, self.variant)
    }
}

impl Validate for Chunk {
    fn validate(&self) -> Result<(), crate::InternalError> {
        self.variant().validate()
    }
}

impl Lines for Chunk {
    fn lines(&self) -> u32 {
        self.variant.lines()
    }
}

impl IntoChunks for Chunk {
    fn into_chunks(self) -> Vec<ast2::Chunk> {
        let (line_no, variant) = self.decompose();

        match variant {
            ChunkVariant::Text(s) => vec![ast2::Chunk::new_unchecked(
                line_no,
                ast2::ChunkVariant::Text(s),
            )],
            ChunkVariant::Scope(sc) => vec![ast2::Chunk::new_unchecked(
                line_no,
                ast2::ChunkVariant::Scope(sc.into()),
            )],
            ChunkVariant::Command(c) => vec![ast2::Chunk::new_unchecked(
                line_no,
                ast2::ChunkVariant::Command(c.into()),
            )],
            ChunkVariant::MathsBlock(b) => b
                .into_chunks()
                .into_iter()
                .map(|mut chunk| {
                    *chunk.line_no_mut() += line_no - 1;
                    chunk
                })
                .collect(),
            ChunkVariant::Environment(env) => vec![ast2::Chunk::new_unchecked(
                line_no,
                ast2::ChunkVariant::Environment(env.into()),
            )],
            ChunkVariant::ParagraphBreak(s) => vec![ast2::Chunk::new_unchecked(
                line_no,
                ast2::ChunkVariant::Text(s),
            )],
        }
    }
}
