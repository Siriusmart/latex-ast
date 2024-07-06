//! # Stage 2 AST
//!
//! The stage 1 AST is folded to create a structure with environments, each chunk can be
//!
//! |Variant|Description|
//! |---|---|
//! |Text|Plain old text without special functionality.|
//! |Scope|Scope to represent grouping of elements.|
//! |Command|`\commandName` followed by multiple arguments.|
//! |Environment|Content between a `\begin{label}{arguments...}` and a `\end{label}`.|
//!
//! The stage 2 AST can be reconstructed a one-to-one copy
//! of the stage 1 AST, and hence the original document with no loss of information.
//!
//! ## Peformance
//!
//! Peformance is highly affected by maximum environment depth.
//!
//! - Worst case O(n<sup>2</sup>) when the document looks like `\begin{a}\begin{a}\begin{a}\end{a}\end{a}\end{a}`
//! - Average case O(n) if the scoping isn't crazy

mod chunk;
mod chunkvariant;
mod command;
mod document;
mod environment;
mod into_chunks;
mod scope;
mod scopevariant;

pub use chunk::Chunk;
pub use chunkvariant::ChunkVariant;
pub use command::Command;
pub use document::Document;
pub use environment::Environment;
pub use into_chunks::IntoChunks;
pub use scope::Scope;
pub use scopevariant::ScopeVariant;
