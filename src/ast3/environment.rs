use crate::ast2;

use super::{Chunk, MathsBlock, Paragraph, Scope};

/// An environment is a scope associated with a command and its arguments
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Environment {
    label: String,
    arguments: Vec<(String, Scope)>, // preceding string, scope
    content: Vec<Chunk>,

    prec_begin: String,
    prec_end: String,
}

impl Environment {
    /// Constructs a new Environment
    pub fn new(
        label: String,
        arguments: Vec<(String, Scope)>,
        content: Vec<Chunk>,
        prec_begin: String,
        prec_end: String,
    ) -> Self {
        Self {
            label,
            arguments,
            content,

            prec_begin,
            prec_end,
        }
    }
}

impl Environment {
    /// Returns label of environment
    pub fn label(&self) -> &String {
        &self.label
    }

    /// Returns the content of `Environment`
    pub fn decompose(self) -> (String, Vec<(String, Scope)>, Vec<Chunk>, String, String) {
        (
            self.label,
            self.arguments,
            self.content,
            self.prec_begin,
            self.prec_end,
        )
    }
}

impl TryFrom<ast2::Environment> for Environment {
    type Error = crate::Error;

    fn try_from(value: ast2::Environment) -> Result<Self, Self::Error> {
        let (label, args, content, prec_begin, prec_end) = value.decompose();

        let mut args_new = Vec::with_capacity(args.len());

        for (prec, scope) in args {
            args_new.push((prec, scope.try_into()?));
        }

        Ok(Self::new(
            label,
            args_new,
            Paragraph::from_chunks(MathsBlock::from_chunks(content)?),
            prec_begin,
            prec_end,
        ))
    }
}
