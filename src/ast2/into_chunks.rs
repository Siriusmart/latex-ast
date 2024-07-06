use super::Chunk;

/// All structs that can be mapped to `ast2::Chunks`
pub trait IntoChunks {
    /// Convert self into a `Vec` of `ast2::Chunks`
    fn into_chunks(self) -> Vec<Chunk>;
}
