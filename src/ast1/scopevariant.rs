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

impl ScopeVariant {
    /// Get the corresponding opening parenthesis,
    /// given ScopeVariant.
    pub fn open(&self) -> char {
        match self {
            Self::Curly => '{',
            Self::Round => '(',
            Self::Square => '[',
        }
    }

    /// Get the corresponding closing parenthesis,
    /// given ScopeVariant.
    pub fn close(&self) -> char {
        match self {
            Self::Curly => '}',
            Self::Round => ')',
            Self::Square => ']',
        }
    }

    /// Whether the current character is a valid opening parenthesis
    pub fn is_opening(c: char) -> bool {
        matches!(c, '{' | '[' | '(')
    }

    /// Whether the current character is a valid closing parenthesis
    pub fn is_closing(c: char) -> bool {
        matches!(c, '}' | ']' | ')')
    }

    /// Return ScopeVariant given an opening character
    pub fn from_opening(c: char) -> Self {
        match c {
            '{' => Self::Curly,
            '[' => Self::Square,
            '(' => Self::Round,
            _ => unreachable!("not an opening"),
        }
    }

    /// Return ScopeVariant given a closing character
    pub fn from_closing(c: char) -> Self {
        match c {
            '}' => Self::Curly,
            ']' => Self::Square,
            ')' => Self::Round,
            _ => unreachable!("not an opening"),
        }
    }
}
