use crate::ast3;

use super::{Chunk, ScopeVariant};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Scope {
    chunks: Vec<Chunk>,
    variant: ScopeVariant,
}

impl From<ast3::Scope> for Scope {
    fn from(value: ast3::Scope) -> Self {
        todo!()
    }
}
