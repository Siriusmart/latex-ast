use crate::ast3;

use super::{Chunk, MathsType, MathsVariant};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct MathsBlock {
    variant: MathsVariant,
    r#type: MathsType,
    content: Vec<Chunk>,
}

impl From<ast3::MathsBlock> for MathsBlock {
    fn from(value: ast3::MathsBlock) -> Self {
        todo!()
    }
}
