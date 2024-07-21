use std::{collections::HashMap, str::FromStr};

use ast3::{Chunk, Command, Environment, MathsBlock, Scope};

use crate::*;

#[test]
fn basic() {
    let content = r#"
\documentclass{article}

\usepackage{amsmath}

\begin{document}
    Hello
    \begin{itemize}{item}
        \item test
        \item test2
        \item test3 $$hello

world$$
    \end{itemize}
\end{document}
        "#
    .trim();

    let ast = ast3::Document::from_str(content).unwrap();

    let expected = ast3::Document::new(
        vec![
            ast3::Chunk::new(
                1,
                ast3::ChunkVariant::Command(
                    Command::new(
                        "documentclass".to_string(),
                        vec![(
                            String::new(),
                            Scope::new(
                                vec![Chunk::new(
                                    1,
                                    ast3::ChunkVariant::Text("article".to_string()),
                                )
                                .unwrap()],
                                ast3::ScopeVariant::Curly,
                            )
                            .unwrap(),
                        )],
                    )
                    .unwrap(),
                ),
            )
            .unwrap(),
            ast3::Chunk::new(1, ast3::ChunkVariant::ParagraphBreak("\n\n".to_string())).unwrap(),
            ast3::Chunk::new(
                3,
                ast3::ChunkVariant::Command(
                    Command::new(
                        "usepackage".to_string(),
                        vec![(
                            String::new(),
                            Scope::new(
                                vec![Chunk::new(
                                    1,
                                    ast3::ChunkVariant::Text("amsmath".to_string()),
                                )
                                .unwrap()],
                                ast3::ScopeVariant::Curly,
                            )
                            .unwrap(),
                        )],
                    )
                    .unwrap(),
                ),
            )
            .unwrap(),
            ast3::Chunk::new(3, ast3::ChunkVariant::ParagraphBreak("\n\n".to_string())).unwrap(),
        ],
        Some("article".to_string()),
        Vec::default(),
        vec![
            Chunk::new(1, ast3::ChunkVariant::Text("\n    Hello\n    ".to_string())).unwrap(),
            Chunk::new(
                3,
                ast3::ChunkVariant::Environment(
                    Environment::new(
                        "itemize".to_string(),
                        vec![(
                            String::new(),
                            Scope::new(
                                vec![Chunk::new(1, ast3::ChunkVariant::Text("item".to_string()))
                                    .unwrap()],
                                ast3::ScopeVariant::Curly,
                            )
                            .unwrap(),
                        )],
                        vec![
                            Chunk::new(1, ast3::ChunkVariant::Text("\n        ".to_string()))
                                .unwrap(),
                            Chunk::new(
                                2,
                                ast3::ChunkVariant::Command(
                                    Command::new("item".to_string(), Vec::new()).unwrap(),
                                ),
                            )
                            .unwrap(),
                            Chunk::new(2, ast3::ChunkVariant::Text(" test\n        ".to_string()))
                                .unwrap(),
                            Chunk::new(
                                3,
                                ast3::ChunkVariant::Command(
                                    Command::new("item".to_string(), Vec::new()).unwrap(),
                                ),
                            )
                            .unwrap(),
                            Chunk::new(3, ast3::ChunkVariant::Text(" test2\n        ".to_string()))
                                .unwrap(),
                            Chunk::new(
                                4,
                                ast3::ChunkVariant::Command(
                                    Command::new("item".to_string(), Vec::new()).unwrap(),
                                ),
                            )
                            .unwrap(),
                            Chunk::new(4, ast3::ChunkVariant::Text(" test3 ".to_string())).unwrap(),
                            Chunk::new(
                                4,
                                ast3::ChunkVariant::MathsBlock(
                                    MathsBlock::new(
                                        ast3::MathsVariant::Dollars,
                                        ast3::MathsType::Outline,
                                        vec![
                                            Chunk::new(
                                                1,
                                                ast3::ChunkVariant::Text("hello".to_string()),
                                            )
                                            .unwrap(),
                                            Chunk::new(
                                                1,
                                                ast3::ChunkVariant::ParagraphBreak(
                                                    "\n\n".to_string(),
                                                ),
                                            )
                                            .unwrap(),
                                            Chunk::new(
                                                3,
                                                ast3::ChunkVariant::Text("world".to_string()),
                                            )
                                            .unwrap(),
                                        ],
                                    )
                                    .unwrap(),
                                ),
                            )
                            .unwrap(),
                            Chunk::new(6, ast3::ChunkVariant::Text("\n    ".to_string())).unwrap(),
                        ],
                        String::new(),
                        String::new(),
                    )
                    .unwrap(),
                ),
            )
            .unwrap(),
            Chunk::new(9, ast3::ChunkVariant::Text("\n".to_string())).unwrap(),
        ],
        Vec::new(),
        String::new(),
        String::new(),
        vec![],
    )
    .unwrap();

    println!("{ast}");
    assert_eq!(content, ast.to_string().as_str());
    assert_eq!(dbg!(expected), dbg!(ast));
}

#[test]
fn begin_command() {
    assert_eq!(
        Err(InternalError::BeginCommand),
        ast3::Command::new("begin".to_string(), Vec::new())
    )
}

#[test]
fn end_command() {
    assert_eq!(
        Err(InternalError::EndCommand),
        ast3::Command::new("end".to_string(), Vec::new())
    )
}

#[test]
fn paragraphbreak_tooshort() {
    assert_eq!(
        Err(InternalError::ParagraphBreakTooShort),
        ast3::Chunk::new(1, ast3::ChunkVariant::ParagraphBreak("\n".to_string()))
    );
}

#[test]
fn paragraphbreak_nonwhitespace() {
    assert_eq!(
        Err(InternalError::UnbrokenParagraph),
        ast3::Chunk::new(1, ast3::ChunkVariant::Text("\n\n".to_string()))
    );
}
