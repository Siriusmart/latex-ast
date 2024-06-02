//! # Warning: unfinished crate
//!
//! Planned stages of AST are
//!
//! - [x] Stage 1: document ➡ vector of chunks
//! - [x] Stage 2: stage 1 ➡ environment-based AST
//! - [ ] Stage 3: stage 2 ➡ handles paragraphs and inline maths

pub mod ast1;
pub mod ast2;

mod error;
pub use error::*;

mod tests;
