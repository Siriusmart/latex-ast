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
