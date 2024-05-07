//! # Warning: unfinished crate
//!
//! Planned stages of AST are
//!
//! - [x] Stage 1: document ➡ vector of chunks
//! - [ ] Stage 2: stage 1 ➡ environment-based AST
//! - [ ] Stage 3: stage 2 ➡ paragraphs and inline maths by inserting virtual commands (?)

pub mod ast1;
mod error;
pub use error::*;

mod tests;
