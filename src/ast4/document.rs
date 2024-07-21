use std::collections::HashMap;

use crate::ast3;

use super::{documentoptions::DocumentOptions, Chunk, DocumentClass, Scope};

#[derive(Default, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Document {
    documentclass: Option<DocumentClass>,
    documentoptions: DocumentOptions,

    preamable: Vec<Chunk>,

    body: Vec<Chunk>,
    body_args: Vec<(String, Scope)>,
    body_begin_prec: String,
    body_end_prec: String,

    trailing: Vec<Chunk>,
}

impl From<ast3::Document> for Document {
    fn from(value: ast3::Document) -> Self {
        todo!()
    }
}
