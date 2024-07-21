//! # Warning: unfinished crate
//!
//! Planned stages of AST are
//!
//! - [x] Stage 1: document ➡ vector of chunks
//! - [x] Stage 2: stage 1 ➡ environment-based AST
//! - [x] Stage 3: stage 2 ➡ handles paragraphs and inline maths
//! - [ ] Stage 4: stage 3 ➡ handling commands and environments depending on type
//!
//! Missing sanitisation checks for line numbers for constructing individual chunks.
//!
//! TODO: Add document option/document class to `Document` when new chunks are pushed.

pub mod ast1;
pub mod ast2;
pub mod ast3;
pub mod ast4;

pub mod traits;

mod error;
pub use error::*;

mod internal_error;
pub use internal_error::InternalError;

mod tests;
