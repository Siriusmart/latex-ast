use crate::ast1;

use super::Scope;

/// Represents a command and its arguments
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Command {
    label: String,
    /// Stored as `Vec<(Preceding string, scope content)>`
    ///
    /// Preceding string is the string between the current argument block and the previous block
    arguments: Vec<(String, Scope)>, // preceding string, scope
}

impl Command {
    /// Maps to `ast1::Command`
    pub fn to_ast1_command(self) -> ast1::Command {
        ast1::Command::new(
            self.label,
            self.arguments
                .into_iter()
                .map(|(s, sc)| (s, sc.to_ast1_scope()))
                .collect(),
        )
    }
}

impl Command {
    /// Constructs a new Command from label and arguments
    pub fn new(label: String, arguments: Vec<(String, Scope)>) -> Self {
        Self { label, arguments }
    }
}

impl TryFrom<crate::ast1::Command> for Command {
    type Error = crate::Error;

    fn try_from(value: crate::ast1::Command) -> Result<Self, Self::Error> {
        let (label, arguments_o) = value.decompose();

        let mut arguments = Vec::with_capacity(arguments_o.len());

        for (prec, scope) in arguments_o {
            arguments.push((prec, scope.try_into()?))
        }

        Ok(Self { label, arguments })
    }
}
