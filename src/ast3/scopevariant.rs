use crate::ast2;

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

impl From<ast2::ScopeVariant> for ScopeVariant {
    fn from(value: ast2::ScopeVariant) -> Self {
        match value {
            ast2::ScopeVariant::Curly => Self::Curly,
            ast2::ScopeVariant::Round => Self::Round,
            ast2::ScopeVariant::Square => Self::Square,
        }
    }
}
