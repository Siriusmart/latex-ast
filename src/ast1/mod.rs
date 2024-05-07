//! # Stage 1 AST
//!
//! The document is parsed into a vector of chunks, each chunk can be
//!
//! |Variant|Description|
//! |---|---|
//! |Text|Plain old text without special functionality.|
//! |Scope|Scope to represent grouping of elements.|
//! |Command|`\commandName` followed by multiple arguments.|
//!
//! The stage 1 AST can be reconstructed a one-to-one copy
//! of the original document with no loss of information.
//!
//! ## Peformance
//!
//! Peformance is highly affected by maximum scope depth.
//!
//! - Worst case O(n<sup>2</sup>) when the document looks like `{{{{{{{{}}}}}}}}`
//! - Average case O(n) if the scoping isn't crazy

mod chunk;
mod chunkvariant;
mod command;
mod document;
mod scope;
mod scopevariant;

pub use chunk::Chunk;
pub use chunkvariant::ChunkVariant;
pub use command::Command;
pub use document::Document;
pub use scope::Scope;
pub use scopevariant::ScopeVariant;
