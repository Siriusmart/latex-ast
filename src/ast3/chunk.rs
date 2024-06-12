use super::ChunkVariant;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Chunk {
    line_no: u32,
    variant: ChunkVariant,
}

impl Chunk {
    /// Constructs new Chunk
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

    /// Returns all fields of this struct
    pub fn decompose(self) -> (u32, ChunkVariant) {
        (self.line_no, self.variant)
    }
}
