use std::str::FromStr;

use crate::{ast1, ast2::Document};

use super::ChunkVariant;

/// A chunk is a block of self contained content
///
/// - `Vec<Chunk>` makes a Document
/// - Each chunk has a line number, indicating the line number its starting character is in
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub struct Chunk {
    line_no: u32,
    variant: ChunkVariant,
}

impl ast1::IntoChunks for Chunk {
    fn into_chunks(self) -> Vec<ast1::Chunk> {
        vec![ast1::Chunk::new(
            self.line_no,
            match self.variant {
                ChunkVariant::Text(s) => ast1::ChunkVariant::Text(s),
                ChunkVariant::Scope(sc) => ast1::ChunkVariant::Scope(sc.to_ast1_scope()),
                ChunkVariant::Command(c) => ast1::ChunkVariant::Command(c.to_ast1_command()),
                ChunkVariant::Environment(env) => {
                    let (label, args, content, prec_begin, prec_end) = env.decompose();

                    fn lines(s: &str) -> usize {
                        s.chars().filter(|c| c == &'\n').count()
                    }

                    let end_line_no = self.line_no
                        + (lines(&prec_begin)
                            + label.chars().filter(|c| c == &'\n').count()
                            + lines(&Document::new(content.clone()).to_string())
                            + args
                                .iter()
                                .map(|(prec, arg)| {
                                    lines(prec)
                                        + lines(
                                            &Document::new(vec![Chunk::new(
                                                1,
                                                ChunkVariant::Scope(arg.clone()),
                                            )])
                                            .to_string(),
                                        )
                                })
                                .sum::<usize>()) as u32;

                    let label = ast1::Scope::new(
                        ast1::Document::from_str(&label).unwrap().chunks_owned(),
                        ast1::ScopeVariant::Curly,
                    );

                    return [ast1::Chunk::new(
                        self.line_no,
                        ast1::ChunkVariant::Command(ast1::Command::new(
                            "begin".to_string(),
                            [(prec_begin, label.clone())]
                                .into_iter()
                                .chain(
                                    args.into_iter()
                                        .map(|(prec, arg)| (prec, arg.to_ast1_scope())),
                                )
                                .collect(),
                        )),
                    )]
                    .into_iter()
                    .chain(content.into_iter().flat_map(|mut chunk| {
                        chunk.line_no += self.line_no - 1;
                        chunk.into_chunks()
                    }))
                    .chain(
                        [ast1::Chunk::new(
                            end_line_no,
                            ast1::ChunkVariant::Command(ast1::Command::new(
                                "end".to_string(),
                                vec![(prec_end, label.clone())],
                            )),
                        )]
                        .into_iter(),
                    )
                    .collect();
                }
            },
        )]
    }
}

impl Chunk {
    pub fn new(line_no: u32, variant: ChunkVariant) -> Self {
        Self { line_no, variant }
    }
}
