use crate::ast3;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Command;

impl From<ast3::Command> for Command {
    fn from(value: ast3::Command) -> Self {
        todo!()
    }
}
