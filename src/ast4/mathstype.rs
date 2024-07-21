use crate::ast3;

/// Type of maths block
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub enum MathsType {
    /// Inline maths is surrounded by $ or \(
    Inline,
    /// Outline maths is surrounded by $$ or \[
    Outline,
}

impl From<ast3::MathsType> for MathsType {
    fn from(value: ast3::MathsType) -> Self {
        todo!()
    }
}
