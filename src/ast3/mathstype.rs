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
