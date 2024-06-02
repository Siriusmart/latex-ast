use std::fmt::Display;

use super::chunkvariant::ChunkVariant;

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
    pub fn new(line_no: u32, variant: ChunkVariant) -> Self {
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

    /// Returns the owned variant of current chunk
    pub fn variant_owned(self) -> ChunkVariant {
        self.variant
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.variant))
    }
}
