use std::fmt::Display;

use crate::ast1::ScopeVariant;

/// Main error struct
///
/// Contains line number of where the error occurs
#[derive(Debug)]
pub struct Error {
    pub line: u32,
    pub r#type: ErrorType,
}

impl Error {
    pub fn new(line: u32, r#type: ErrorType) -> Self {
        Self { line, r#type }
    }
}

/// Error message content
#[derive(Debug)]
pub enum ErrorType {
    /// There are too many closing parenthesis of said variant
    UnexpectedClosing(ScopeVariant),
    /// There is an unclosed command argument of said parenthesis variant
    UnclosedArgument(ScopeVariant),
    /// There is an unclosed scope of said parenthesis variant
    UnclosedScope(ScopeVariant),
    /// Missing environment label
    NoEnvironmentLabel,
    /// There are too many `\end` of said environment
    UnexpectedEnd(String),
    /// The environment is unclosed with a `\end` command
    UnclosedEnvironment(String),
    /// There are more than one argument at a `\end` command
    TooManyArgsEnd,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl std::error::Error for Error {}
