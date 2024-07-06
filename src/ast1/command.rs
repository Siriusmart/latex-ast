use std::fmt::Display;
use std::fmt::Write;

use crate::ast1::ScopeVariant;
use crate::ast2;
use crate::traits::Lines;
use crate::traits::Validate;
use crate::InternalError;

use super::scope::Scope;

/// Represents a command and its arguments
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Command {
    /// Name of the command
    label: String,
    /// Stored as `Vec<(Preceding string, scope content)>`
    ///
    /// Preceding string is the string between the current argument block and the previous block
    arguments: Vec<(String, Scope)>, // preceding string, scope
}

impl Command {
    /// Construct new Command
    pub fn new(label: String, arguments: Vec<(String, Scope)>) -> Result<Self, InternalError> {
        let out = Self { label, arguments };
        out.validate()?;
        Ok(out)
    }

    /// Construct new Command without checking
    pub fn new_unchecked(label: String, arguments: Vec<(String, Scope)>) -> Self {
        Self { label, arguments }
    }

    /// Return label of the command
    pub fn label(&self) -> &String {
        &self.label
    }

    /// Return argument of the command
    pub fn arguments(&self) -> &Vec<(String, Scope)> {
        &self.arguments
    }

    /// Return owned label of the command
    pub fn label_owned(self) -> String {
        self.label
    }

    /// Return owned arguments of the command
    pub fn arguments_owned(self) -> Vec<(String, Scope)> {
        self.arguments
    }

    /// Returns all fields of this struct
    pub fn decompose(self) -> (String, Vec<(String, Scope)>) {
        (self.label, self.arguments)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\\{}{}",
            self.label,
            self.arguments
                .iter()
                .fold(String::new(), |mut s, (scope, trailing)| {
                    let _ = write!(s, "{scope}{trailing}");
                    s
                })
        ))
    }
}

impl Validate for Command {
    fn validate(&self) -> Result<(), crate::InternalError> {
        if self.label.len() != 1 {
            for c in self.label.chars() {
                if matches!(c, '\\' | '%')
                    || ScopeVariant::is_opening(c)
                    || ScopeVariant::is_closing(c)
                {
                    return Err(crate::InternalError::UnsanitisedCharInString(c));
                }
            }
        }

        for (_, arg) in self.arguments.iter() {
            arg.validate()?
        }

        Ok(())
    }
}

impl Lines for Command {
    fn lines(&self) -> u32 {
        let mut total = self.label.chars().filter(|c| c == &'\n').count() as u32;

        for (prec, arg) in self.arguments.iter() {
            total += prec.chars().filter(|c| c == &'\n').count() as u32;
            total += arg.lines() - 1
        }

        total + 1
    }
}

impl From<ast2::Command> for Command {
    fn from(value: ast2::Command) -> Self {
        let (label, arguments) = value.decompose();
        Command::new_unchecked(
            label,
            arguments
                .into_iter()
                .map(|(s, sc)| (s, sc.to_ast1_scope()))
                .collect(),
        )
    }
}
