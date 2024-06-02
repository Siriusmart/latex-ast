use super::{Chunk, Scope};

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
