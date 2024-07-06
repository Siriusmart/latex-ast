//! # Warning: unfinished crate
//!
//! Planned stages of AST are
//!
//! - [x] Stage 1: document ➡ vector of chunks
//! - [x] Stage 2: stage 1 ➡ environment-based AST
//! - [x] Stage 3: stage 2 ➡ handles paragraphs and inline maths
//! - [ ] Stage 4: stage 3 ➡ programmatic handling/querying of chunks
//!
//! Missing sanitisation checks for line numbers for constructing individual chunks.

pub mod ast1;
pub mod ast2;
pub mod ast3;

pub mod traits;

mod error;
pub use error::*;

mod internal_error;
pub use internal_error::InternalError;

mod tests;
