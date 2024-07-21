use crate::ast3;

/// Variant used to declare the maths block
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub enum MathsVariant {
    /// \( or \[
    Brackets,
    /// $ or $$
    Dollars,
}

impl From<ast3::MathsVariant> for MathsVariant {
    fn from(value: ast3::MathsVariant) -> Self {
        todo!()
    }
}
