use crate::ast3;

use super::ChunkVariant;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Chunk {
    line_no: u32,
    variant: ChunkVariant,
}

impl From<ast3::Chunk> for Chunk {
    fn from(value: ast3::Chunk) -> Self {
        todo!()
    }
}
