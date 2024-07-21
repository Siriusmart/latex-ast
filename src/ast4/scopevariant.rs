use crate::ast3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeVariant {
    /// `{}`
    Curly,
    /// `()`
    Round,
    /// `[]`
    Square,
}

impl From<ast3::ScopeVariant> for ScopeVariant {
    fn from(value: ast3::ScopeVariant) -> Self {
        todo!()
    }
}
