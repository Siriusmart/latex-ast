use std::fmt::Display;

use crate::{
    ast1::{self, IntoChunks},
    ast3,
    traits::{Lines, Validate},
    InternalError,
};

use super::{Chunk, ChunkVariant, IntoChunks as IntoChunks3, Scope};

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

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            ast1::Document::new_unchecked(
                Chunk::new_unchecked(1, ChunkVariant::Environment(self.clone())).into_chunks()
            )
        ))
    }
}

impl Environment {
    pub fn new(
        label: String,
        arguments: Vec<(String, Scope)>,
        content: Vec<Chunk>,
        prec_begin: String,
        prec_end: String,
    ) -> Result<Self, InternalError> {
        let out = Self {
            label,
            arguments,
            content,

            prec_begin,
            prec_end,
        };

        out.validate()?;

        Ok(out)
    }

    /// Constructs a new Environment
    pub fn new_unchecked(
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

impl Validate for Environment {
    fn validate(&self) -> Result<(), crate::InternalError> {
        for c in self.label.chars() {
            if matches!(c, '\\' | '%')
                || ast1::ScopeVariant::is_opening(c)
                || ast1::ScopeVariant::is_closing(c)
            {
                return Err(crate::InternalError::UnsanitisedCharInString(c));
            }
        }

        for (_, arg) in self.arguments.iter() {
            arg.validate()?
        }

        for chunk in self.content.iter() {
            chunk.validate()?
        }

        Ok(())
    }
}

impl Lines for Environment {
    fn lines(&self) -> u32 {
        let mut lines = self.label.chars().filter(|c| c == &'\n').count() as u32 * 2;
        lines += self
            .prec_begin
            .chars()
            .chain(self.prec_end.chars())
            .filter(|c| c == &'\n')
            .count() as u32;

        for (prec, arg) in self.arguments.iter() {
            lines += prec.chars().filter(|c| c == &'\n').count() as u32;
            lines += arg.lines() - 1;
        }

        for chunk in self.content.iter() {
            lines += chunk.lines() - 1;
        }

        lines + 1
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

impl From<ast3::Environment> for Environment {
    fn from(value: ast3::Environment) -> Self {
        let (label, arguments, content, prec_begin, prec_end) = value.decompose();

        Self::new_unchecked(
            label,
            arguments
                .into_iter()
                .map(|(s, chunk)| (s, chunk.into()))
                .collect(),
            content
                .into_iter()
                .flat_map(|chunk| chunk.into_chunks())
                .collect(),
            prec_begin,
            prec_end,
        )
    }
}
