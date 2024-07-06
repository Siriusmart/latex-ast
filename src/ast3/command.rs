use std::fmt::Display;
use std::fmt::Write;

use crate::{
    ast1, ast2,
    traits::{Lines, Validate},
    InternalError,
};

use super::Scope;

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

impl TryFrom<ast2::Command> for Command {
    type Error = crate::Error;

    fn try_from(value: ast2::Command) -> Result<Self, Self::Error> {
        let (label, args) = value.decompose();

        let mut args_new = Vec::with_capacity(args.len());

        for (prec, scope) in args {
            args_new.push((prec, scope.try_into()?));
        }

        Ok(Self::new_unchecked(label, args_new))
    }
}

impl Validate for Command {
    fn validate(&self) -> Result<(), crate::InternalError> {
        match self.label.as_str() {
            "begin" => return Err(crate::InternalError::BeginCommand),
            "end" => return Err(crate::InternalError::EndCommand),
            _ => {}
        }

        if self.label.len() != 1 {
            for c in self.label.chars() {
                if matches!(c, '\\' | '%')
                    || ast1::ScopeVariant::is_opening(c)
                    || ast1::ScopeVariant::is_closing(c)
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
