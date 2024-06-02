use super::Chunk;

/// All structs that can be mapped to `ast1::Chunks`
pub trait IntoChunks {
    fn into_chunks(self) -> Vec<Chunk>;
}
