//! # Warning: unfinished crate
//!
//! Planned stages of AST are
//!
//! - [x] Stage 1: document ➡ vector of chunks
//! - [x] Stage 2: stage 1 ➡ environment-based AST
//! - [ ] Stage 3: stage 2 ➡ handles paragraphs and inline maths
//!
//! Currently there are no sanitisation checks for modifications to ASTs,
//! please do so with extreme caution.

pub mod ast1;
pub mod ast2;
pub mod ast3;

mod error;
pub use error::*;

mod tests;
