//! # Stage 3 AST
//!
//! The stage 2 AST is broken down to include maths blocks and paragraphs.
//!
//! |Variant|Description|
//! |---|---|
//! |Text|Plain old text without special functionality.|
//! |Scope|Scope to represent grouping of elements.|
//! |Command|`\commandName` followed by multiple arguments.|
//! |Environment|Content between a `\begin{label}{arguments...}` and a `\end{label}`.|
//! |MathsBlock|Content surrounded by `$`, `\[` or `\(`.|
//! |InterParagraph|A paragraph break.|
//!
//! The stage 3 AST can be reconstructed a one-to-one copy
//! of the stage 2 AST, and hence the original document with no loss of information.
//!
//! ## Peformance
//!
//! Peformance is not known, current assuming it to be
//! - Worst case O(n<sup>2</sup>)
//! - Average case O(n)

mod chunk;
mod chunkvariant;
mod command;
mod document;
mod environment;
mod mathsblock;
mod mathstype;
mod mathsvariant;
mod paragraph;
mod scope;
mod scopevariant;

pub use chunk::Chunk;
pub use chunkvariant::ChunkVariant;
pub use command::Command;
pub use document::Document;
pub use environment::Environment;
pub use mathsblock::MathsBlock;
pub use mathstype::MathsType;
pub use mathsvariant::MathsVariant;
pub use paragraph::Paragraph;
pub use scope::Scope;
pub use scopevariant::ScopeVariant;
