use std::{collections::HashMap, str::FromStr};

use ast3::{Chunk, Command, Environment, Scope};

use crate::*;

#[test]
fn simple() {
    let content = r#"
\documentclass{article}

\usepackage{amsmath}

\begin{document}
    Hello
    \begin{itemize}
        \item test
        \item test2
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
        HashMap::default(),
        vec![
            Chunk::new(1, ast3::ChunkVariant::Text("\n    Hello\n    ".to_string())).unwrap(),
            Chunk::new(
                3,
                ast3::ChunkVariant::Environment(
                    Environment::new(
                        "itemize".to_string(),
                        Vec::new(),
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
                            Chunk::new(3, ast3::ChunkVariant::Text(" test2\n    ".to_string()))
                                .unwrap(),
                        ],
                        String::new(),
                        String::new(),
                    )
                    .unwrap(),
                ),
            )
            .unwrap(),
            Chunk::new(6, ast3::ChunkVariant::Text("\n".to_string())).unwrap(),
        ],
        Vec::new(),
        String::new(),
        String::new(),
        vec![],
    )
    .unwrap();

    assert_eq!(dbg!(expected), dbg!(ast));
}
