use std::{collections::HashMap, str::FromStr};

use crate::{
    ast2,
    ast3::{Environment, MathsBlock, Paragraph},
};

use super::{Chunk, Scope};

#[derive(Default, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Document {
    documentclass: Option<String>,
    documentoptions: HashMap<String, Option<String>>,

    preamable: Vec<Chunk>,

    body: Vec<Chunk>,
    body_line: u32,
    body_args: Vec<(String, Scope)>,
    body_begin_prec: String,
    body_end_prec: String,

    trailing: Vec<Chunk>,
}

impl Document {
    pub fn new(
        preamable: Vec<Chunk>,
        documentclass: Option<String>,
        documentoptions: HashMap<String, Option<String>>,
        body: Vec<Chunk>,
        body_line: u32,
        body_args: Vec<(String, Scope)>,
        body_begin_prec: String,
        body_end_prec: String,
        trailing: Vec<Chunk>,
    ) -> Self {
        Self {
            documentclass,
            documentoptions,
            preamable,
            body,
            body_line,
            body_args,
            body_begin_prec,
            body_end_prec,
            trailing,
        }
    }

    pub fn decompose(
        self,
    ) -> (
        Option<String>,
        HashMap<String, Option<String>>,
        Vec<Chunk>,
        Vec<Chunk>,
        u32,
        Vec<(String, Scope)>,
        String,
        String,
        Vec<Chunk>,
    ) {
        (
            self.documentclass,
            self.documentoptions,
            self.preamable,
            self.body,
            self.body_line,
            self.body_args,
            self.body_begin_prec,
            self.body_end_prec,
            self.trailing,
        )
    }
}

impl TryFrom<ast2::Document> for Document {
    type Error = crate::Error;

    fn try_from(value: ast2::Document) -> Result<Self, Self::Error> {
        let mut construct = Self::default();

        let mut preamable = Vec::new();
        let mut trailing = Vec::new();

        #[derive(PartialEq)]
        enum CursorState {
            Preamable,
            Trailing,
        }

        let mut cursor = CursorState::Preamable;

        for chunk in value.chunks_owned() {
            let (line_no, variant) = chunk.decompose();

            match variant {
                ast2::ChunkVariant::Environment(env) if env.label().as_str() == "document" => {
                    let (_, args, content, begin, end) = Environment::try_from(env)?.decompose();
                    construct.body_args = args;
                    construct.body = content;
                    construct.body_line = line_no;
                    construct.body_begin_prec = begin;
                    construct.body_end_prec = end;
                    cursor = CursorState::Trailing;
                }
                ast2::ChunkVariant::Command(ref cmd)
                    if cmd.label().as_str() == "documentclass"
                        && cursor == CursorState::Preamable =>
                {
                    if construct.documentclass.is_some() {
                        return Err(crate::Error::new(
                            line_no,
                            crate::ErrorType::DoubleDocumentClass,
                        ));
                    }
                    for (_, scope) in cmd.clone().decompose().1 {
                        match scope.variant() {
                            ast2::ScopeVariant::Curly if construct.documentclass.is_some() => {
                                return Err(crate::Error::new(
                                    line_no,
                                    crate::ErrorType::TooManyArgsDocumentClass,
                                ))
                            }
                            ast2::ScopeVariant::Curly => {
                                construct.documentclass =
                                    Some(ast2::Document::new(scope.chunks_owned()).to_string())
                            }
                            _ => {
                                for opt in ast2::Document::new(scope.chunks_owned())
                                    .to_string()
                                    .split(',')
                                {
                                    if let Some((k, v)) = opt.split_once('=') {
                                        construct.documentoptions.insert(
                                            k.trim().to_string(),
                                            Some(v.trim().to_string()),
                                        );
                                    } else {
                                        construct
                                            .documentoptions
                                            .insert(opt.trim().to_string(), None);
                                    }
                                }
                            }
                        }
                    }
                    preamable.push(ast2::Chunk::new(line_no, variant))
                }
                _ if cursor == CursorState::Preamable => {
                    preamable.push(ast2::Chunk::new(line_no, variant))
                }
                _ => trailing.push(ast2::Chunk::new(line_no, variant)),
            }
        }

        construct.preamable = Paragraph::from_chunks(MathsBlock::from_chunks(preamable)?);
        construct.trailing = Paragraph::from_chunks(MathsBlock::from_chunks(trailing)?);

        Ok(construct)
    }
}

impl FromStr for Document {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ast2::Document::from_str(s)?.try_into()
    }
}
