use crate::ast3;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Environment;

impl From<ast3::Environment> for Environment {
    fn from(value: ast3::Environment) -> Self {
        todo!()
    }
}
