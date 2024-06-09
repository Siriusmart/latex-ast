use std::fmt::Display;
use std::fmt::Write;

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
    pub fn new(label: String, arguments: Vec<(String, Scope)>) -> Self {
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
