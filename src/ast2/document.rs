use crate::ast1;

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
    /// Constructs a new Document from chunks
    pub fn new(chunks: Vec<Chunk>) -> Self {
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
}

impl FromStr for Document {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(ast1::Document::from_str(s)?)
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ast1::Document::from(self.clone()).fmt(f)
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
                    chunks.push(Chunk::new(line_no, $x))
                };
            }

            macro_rules! push_buffer {
                ($x:expr) => {
                    buffer.push(ast1::Chunk::new(line_no - buffer_start + 1, $x))
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
                                    ast1::Document::new(
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

                                chunks.push(Chunk::new(
                                    buffer_start,
                                    ChunkVariant::Environment(Environment::new(
                                        ast1::Document::new(label.chunks_owned()).to_string(),
                                        args_new,
                                        map_env_e!(Document::try_from(ast1::Document::new(
                                            mem::take(&mut buffer)
                                        )))
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
                    ast1::Document::new(label.clone()).to_string(),
                ),
            ));
        }

        Ok(Self(chunks))
    }
}
