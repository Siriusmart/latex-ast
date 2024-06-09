use super::Chunk;

/// All structs that can be mapped to `ast1::Chunks`
pub trait IntoChunks {
    /// Convert self into a `Vec` of `ast::Chunks`
    fn into_chunks(self) -> Vec<Chunk>;
}
