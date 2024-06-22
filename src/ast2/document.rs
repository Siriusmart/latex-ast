use crate::{
    ast1, ast3,
    traits::{Lines, Validate},
    InternalError,
};

use super::{Chunk, ChunkVariant, Environment};

use std::{fmt::Display, mem, str::FromStr};

/// Main struct for stage 2 AST
///
/// Display `{}` reconstructs the original document
#[derive(Default, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Document(Vec<Chunk>);

impl Document {
    /// Create new document from chunks
    pub fn new(chunks: Vec<Chunk>) -> Result<Self, InternalError> {
        let out = Self(chunks);
        out.validate()?;
        Ok(out)
    }

    /// Create new document from chunks without checking
    pub fn new_unchecked(chunks: Vec<Chunk>) -> Self {
        Self(chunks)
    }

    /// Returns the chunks inside the document
    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.0
    }

    /// Returns the owned chunks inside the document
    pub fn chunks_owned(self) -> Vec<Chunk> {
        self.0
    }

    /// Push a variant to the document
    pub fn push(&mut self, variant: ChunkVariant) -> Result<(), InternalError> {
        variant.validate()?;
        self.push_unchecked(variant);
        Ok(())
    }

    /// Push a variant to the document without checking
    pub fn push_unchecked(&mut self, variant: ChunkVariant) {
        let line_no = self
            .chunks()
            .last()
            .map(|last| last.line_no() + last.lines() - 1)
            .unwrap_or(1);
        self.push_chunk_unchecked(Chunk::new_unchecked(line_no, variant));
    }

    /// Push a chunk to the document
    pub fn push_chunk(&mut self, chunk: Chunk) -> Result<(), InternalError> {
        chunk.validate()?;

        let expected_line = self
            .chunks()
            .last()
            .map(|last| last.line_no() + last.lines() - 1)
            .unwrap_or(1);

        if expected_line != chunk.line_no() {
            return Err(crate::InternalError::IncorrectChunkLineNumber {
                expected: expected_line,
                got: chunk.line_no(),
            });
        }

        self.push_chunk_unchecked(chunk);
        Ok(())
    }

    /// Push a chunk to the document without checking
    pub fn push_chunk_unchecked(&mut self, chunk: Chunk) {
        if let ChunkVariant::Text(s) = chunk.variant() {
            if let Some(last) = self.0.last_mut() {
                if let ChunkVariant::Text(last_s) = last.variant_mut() {
                    last_s.push_str(s);
                    return;
                }
            }
        }

        self.0.push(chunk)
    }
}

impl FromStr for Document {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(ast1::Document::from_str(s)?)
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter().map(ToString::to_string).collect::<String>())
    }
}

impl Validate for Document {
    fn validate(&self) -> Result<(), crate::InternalError> {
        let mut expected_line = 1;

        for chunk in self.0.iter() {
            if chunk.line_no() != expected_line {
                return Err(InternalError::IncorrectChunkLineNumber {
                    expected: expected_line,
                    got: chunk.line_no(),
                });
            }

            chunk.validate()?;

            expected_line += chunk.lines() - 1;
        }

        Ok(())
    }
}

impl Lines for Document {
    fn lines(&self) -> u32 {
        self.chunks()
            .iter()
            .map(|chunk| chunk.lines() - 1)
            .sum::<u32>()
            + 1
    }
}

impl From<ast3::Document> for Document {
    fn from(value: ast3::Document) -> Self {
        let (_, _, preamable, body, _, body_args, body_begin_prec, body_end_prec, trailing) =
            value.decompose();
        todo!()
    }
}

impl TryFrom<crate::ast1::Document> for Document {
    type Error = crate::Error;
    fn try_from(value: crate::ast1::Document) -> Result<Self, Self::Error> {
        let originals = value.chunks_owned();
        let mut chunks = Vec::new();

        let mut buffer_stack = Vec::new();
        let mut buffer_start = 0;
        let mut buffer: Vec<ast1::Chunk> = Vec::new();
        let mut prec_begin = String::new();

        for original in originals {
            let line_no = original.line_no();

            macro_rules! push_chunks {
                ($x:expr) => {
                    chunks.push(Chunk::new_unchecked(line_no, $x))
                };
            }

            macro_rules! push_buffer {
                ($x:expr) => {
                    buffer.push(ast1::Chunk::new_unchecked(line_no - buffer_start + 1, $x))
                };
            }

            macro_rules! map_e {
                ($x:expr) => {
                    match $x {
                        Ok(k) => k,
                        Err(mut e) => {
                            e.line += line_no;
                            return Err(e);
                        }
                    }
                };
            }

            macro_rules! map_env_e {
                ($x:expr) => {
                    match $x {
                        Ok(k) => k,
                        Err(mut e) => {
                            e.line += buffer_start;
                            return Err(e);
                        }
                    }
                };
            }

            match original.variant_owned() {
                ast1::ChunkVariant::Text(s) if buffer_stack.is_empty() => {
                    push_chunks!(ChunkVariant::Text(s))
                }
                ast1::ChunkVariant::Text(s) => push_buffer!(ast1::ChunkVariant::Text(s)),
                ast1::ChunkVariant::Scope(s) if buffer_stack.is_empty() => {
                    push_chunks!(ChunkVariant::Scope(map_e!(s.try_into())))
                }
                ast1::ChunkVariant::Scope(s) => push_buffer!(ast1::ChunkVariant::Scope(s)),
                ast1::ChunkVariant::Command(c) => {
                    if !matches!(c.label().as_str(), "begin" | "end") {
                        if buffer_stack.is_empty() {
                            push_chunks!(ChunkVariant::Command(c.try_into()?));
                        } else {
                            push_buffer!(ast1::ChunkVariant::Command(c));
                        }
                        continue;
                    }

                    if !c
                        .arguments()
                        .first()
                        .is_some_and(|arg| arg.1.variant() == ast1::ScopeVariant::Curly)
                    {
                        return Err(crate::Error {
                            line: line_no,
                            r#type: crate::ErrorType::NoEnvironmentLabel,
                        });
                    }

                    match c.label().as_str() {
                        "begin" => {
                            let (prec, content) = c.arguments().first().unwrap();

                            buffer_stack.push(content.chunks().clone());

                            if buffer_stack.len() == 1 {
                                // was empty

                                fn lines(s: &str) -> usize {
                                    s.chars().filter(|c| c == &'\n').count()
                                }

                                buffer_start = line_no
                                    + c.arguments()
                                        .iter()
                                        .map(|(prec, arg)| lines(prec) + lines(&arg.to_string()))
                                        .sum::<usize>()
                                        as u32;
                                prec_begin = prec.to_string();
                            } else {
                                push_buffer!(ast1::ChunkVariant::Command(c));
                            }
                        }
                        "end"
                            if buffer_stack.last()
                                != Some(c.arguments().first().unwrap().1.chunks()) =>
                        {
                            return Err(crate::Error::new(
                                line_no,
                                crate::ErrorType::UnexpectedEnd(
                                    ast1::Document::new_unchecked(
                                        c.arguments_owned().remove(0).1.chunks_owned(),
                                    )
                                    .to_string(),
                                ),
                            ))
                        }
                        "end" => {
                            if c.arguments().len() > 1 {
                                return Err(crate::Error::new(
                                    line_no,
                                    crate::ErrorType::TooManyArgsEnd,
                                ));
                            }

                            buffer_stack.pop().unwrap();

                            if buffer_stack.is_empty() {
                                let mut arguments = c.arguments_owned();

                                let (prec_end, label) = arguments.remove(0);
                                let mut args_new = Vec::with_capacity(arguments.len());

                                for (prec, scope) in arguments {
                                    args_new.push((prec, scope.try_into()?))
                                }

                                chunks.push(Chunk::new_unchecked(
                                    buffer_start,
                                    ChunkVariant::Environment(Environment::new_unchecked(
                                        ast1::Document::new_unchecked(label.chunks_owned())
                                            .to_string(),
                                        args_new,
                                        map_env_e!(Document::try_from(
                                            ast1::Document::new_unchecked(mem::take(&mut buffer))
                                        ))
                                        .chunks_owned(),
                                        mem::take(&mut prec_begin),
                                        prec_end,
                                    )),
                                ))
                            } else {
                                push_buffer!(ast1::ChunkVariant::Command(c));
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        if let Some(label) = buffer_stack.first() {
            return Err(crate::Error::new(
                buffer_start,
                crate::ErrorType::UnclosedEnvironment(
                    ast1::Document::new_unchecked(label.clone()).to_string(),
                ),
            ));
        }

        Ok(Self(chunks))
    }
}
