/// Type of parenthesis used for the scope
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeVariant {
    /// `{}`
    Curly,
    /// `()`
    Round,
    /// `[]`
    Square,
}

impl From<crate::ast1::ScopeVariant> for ScopeVariant {
    fn from(value: crate::ast1::ScopeVariant) -> Self {
        match value {
            crate::ast1::ScopeVariant::Curly => Self::Curly,
            crate::ast1::ScopeVariant::Round => Self::Round,
            crate::ast1::ScopeVariant::Square => Self::Square,
        }
    }
}
