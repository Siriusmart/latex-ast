use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::{
    ast2,
    ast3::{Environment, MathsBlock, Paragraph},
    traits::{Lines, Validate},
    InternalError,
};

use super::{Chunk, ChunkVariant, Scope};

#[derive(Default, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Document {
    documentclass: Option<String>,
    documentoptions: HashMap<String, Option<String>>,

    preamable: Vec<Chunk>,

    body: Vec<Chunk>,
    body_args: Vec<(String, Scope)>,
    body_begin_prec: String,
    body_end_prec: String,

    trailing: Vec<Chunk>,
}

impl Validate for Document {
    fn validate(&self) -> Result<(), crate::InternalError> {
        for section in [&self.preamable, &self.body, &self.trailing] {
            let mut expected_line = 1;

            for chunk in section.iter() {
                if chunk.line_no() != expected_line {
                    return Err(InternalError::IncorrectChunkLineNumber {
                        expected: expected_line,
                        got: chunk.line_no(),
                    });
                }

                chunk.validate()?;

                expected_line += chunk.lines() - 1;
            }
        }

        Ok(())
    }
}

impl Lines for Document {
    fn lines(&self) -> u32 {
        self.preamable
            .iter()
            .chain(self.body.iter())
            .chain(self.trailing.iter())
            .map(|chunk| chunk.lines() - 1)
            .sum::<u32>()
            + 1
    }
}

impl Document {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        preamable: Vec<Chunk>,
        documentclass: Option<String>,
        documentoptions: HashMap<String, Option<String>>,
        body: Vec<Chunk>,
        body_args: Vec<(String, Scope)>,
        body_begin_prec: String,
        body_end_prec: String,
        trailing: Vec<Chunk>,
    ) -> Result<Self, InternalError> {
        let out = Self::new_unchecked(
            preamable,
            documentclass,
            documentoptions,
            body,
            body_args,
            body_begin_prec,
            body_end_prec,
            trailing,
        );
        out.validate()?;
        Ok(out)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_unchecked(
        preamable: Vec<Chunk>,
        documentclass: Option<String>,
        documentoptions: HashMap<String, Option<String>>,
        body: Vec<Chunk>,
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
            body_args,
            body_begin_prec,
            body_end_prec,
            trailing,
        }
    }

    pub fn chunks_preamable(&self) -> &Vec<Chunk> {
        &self.preamable
    }

    pub fn chunks_body(&self) -> &Vec<Chunk> {
        &self.body
    }

    pub fn chunks_trailing(&self) -> &Vec<Chunk> {
        &self.trailing
    }

    fn push_chunk_vec_unchecked(vec: &mut Vec<Chunk>, chunk: Chunk) {
        if let ChunkVariant::Text(s) = chunk.variant() {
            if let Some(last) = vec.last_mut() {
                if let ChunkVariant::Text(last_s) = last.variant_mut() {
                    last_s.push_str(s);
                    return;
                }
            }
        }

        vec.push(chunk)
    }

    fn push_chunk_vec(vec: &mut Vec<Chunk>, chunk: Chunk) -> Result<(), InternalError> {
        chunk.validate()?;

        let expected_line = vec
            .last()
            .map(|last| last.line_no() + last.lines() - 1)
            .unwrap_or(1);

        if expected_line != chunk.line_no() {
            return Err(crate::InternalError::IncorrectChunkLineNumber {
                expected: expected_line,
                got: chunk.line_no(),
            });
        }

        Self::push_chunk_vec_unchecked(vec, chunk);
        Ok(())
    }

    fn push_vec_unchecked(vec: &mut Vec<Chunk>, variant: ChunkVariant) {
        let line_no = vec
            .last()
            .map(|last| last.line_no() + last.lines() - 1)
            .unwrap_or(1);
        Self::push_chunk_vec_unchecked(vec, Chunk::new_unchecked(line_no, variant));
    }

    fn push_vec(vec: &mut Vec<Chunk>, variant: ChunkVariant) -> Result<(), InternalError> {
        variant.validate()?;
        Self::push_vec_unchecked(vec, variant);
        Ok(())
    }

    pub fn push_body(&mut self, variant: ChunkVariant) -> Result<(), InternalError> {
        Self::push_vec(&mut self.body, variant)
    }

    pub fn push_body_unchecked(&mut self, variant: ChunkVariant) {
        Self::push_vec_unchecked(&mut self.body, variant)
    }

    pub fn push_body_chunk(&mut self, chunk: Chunk) -> Result<(), InternalError> {
        Self::push_chunk_vec(&mut self.body, chunk)
    }

    pub fn push_body_chunk_unchecked(&mut self, chunk: Chunk) {
        Self::push_chunk_vec_unchecked(&mut self.body, chunk)
    }

    pub fn push_preamable(&mut self, variant: ChunkVariant) -> Result<(), InternalError> {
        Self::push_vec(&mut self.preamable, variant)
    }

    pub fn push_preamable_unchecked(&mut self, variant: ChunkVariant) {
        Self::push_vec_unchecked(&mut self.preamable, variant)
    }

    pub fn push_preamable_chunk(&mut self, chunk: Chunk) -> Result<(), InternalError> {
        Self::push_chunk_vec(&mut self.preamable, chunk)
    }

    pub fn push_preamable_chunk_unchecked(&mut self, chunk: Chunk) {
        Self::push_chunk_vec_unchecked(&mut self.preamable, chunk)
    }

    pub fn push_trailing(&mut self, variant: ChunkVariant) -> Result<(), InternalError> {
        Self::push_vec(&mut self.trailing, variant)
    }

    pub fn push_trailing_unchecked(&mut self, variant: ChunkVariant) {
        Self::push_vec_unchecked(&mut self.trailing, variant)
    }

    pub fn push_trailing_chunk(&mut self, chunk: Chunk) -> Result<(), InternalError> {
        Self::push_chunk_vec(&mut self.trailing, chunk)
    }

    pub fn push_trailing_chunk_unchecked(&mut self, chunk: Chunk) {
        Self::push_chunk_vec_unchecked(&mut self.trailing, chunk)
    }

    #[allow(clippy::type_complexity)]
    pub fn decompose(
        self,
    ) -> (
        Option<String>,
        HashMap<String, Option<String>>,
        Vec<Chunk>,
        Vec<Chunk>,
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
                                construct.documentclass = Some(
                                    ast2::Document::new_unchecked(scope.chunks_owned()).to_string(),
                                )
                            }
                            _ => {
                                for opt in ast2::Document::new_unchecked(scope.chunks_owned())
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
                    preamable.push(ast2::Chunk::new_unchecked(line_no, variant))
                }
                _ if cursor == CursorState::Preamable => {
                    preamable.push(ast2::Chunk::new_unchecked(line_no, variant))
                }
                _ => trailing.push(ast2::Chunk::new_unchecked(line_no, variant)),
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

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", ast2::Document::from(self.clone())))
    }
}
