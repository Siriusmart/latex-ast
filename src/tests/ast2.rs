use std::str::FromStr;

use crate::{
    ast1,
    ast2::{Chunk, ChunkVariant, Command, Document, Environment, Scope, ScopeVariant},
};

#[test]
fn basic() {
    let content = r#"
\usepackage{amsmath}

\begin{document}
    Hello
    \begin{itemize}
        \item test
    \end{itemize}
\end{document}
        "#
    .trim();

    let one = ast1::Document::from_str(content).unwrap();
    let two = Document::from_str(content).unwrap();

    assert_eq!(
        dbg!(&two),
        dbg!(&Document::new(vec![
            Chunk::new(
                1,
                ChunkVariant::Command(Command::new(
                    "usepackage".to_string(),
                    vec![(
                        "".to_string(),
                        Scope::new(
                            vec![Chunk::new(1, ChunkVariant::Text("amsmath".to_string()))],
                            ScopeVariant::Curly
                        )
                    )]
                ))
            ),
            Chunk::new(1, ChunkVariant::Text("\n\n".to_string())),
            Chunk::new(
                3,
                ChunkVariant::Environment(Environment::new(
                    "document".to_string(),
                    Vec::new(),
                    vec![
                        Chunk::new(1, ChunkVariant::Text("\n    Hello\n    ".to_string())),
                        Chunk::new(
                            3,
                            ChunkVariant::Environment(Environment::new(
                                "itemize".to_string(),
                                Vec::new(),
                                vec![
                                    Chunk::new(1, ChunkVariant::Text("\n        ".to_string())),
                                    Chunk::new(
                                        2,
                                        ChunkVariant::Command(Command::new(
                                            "item".to_string(),
                                            Vec::new()
                                        ))
                                    ),
                                    Chunk::new(2, ChunkVariant::Text(" test\n    ".to_string()))
                                ],
                                String::new(),
                                String::new()
                            ))
                        ),
                        Chunk::new(5, ChunkVariant::Text("\n".to_string()))
                    ],
                    String::new(),
                    String::new()
                ))
            ),
        ]))
    );

    assert_eq!(dbg!(one), dbg!(ast1::Document::from(two.clone())));
    assert_eq!(content, two.to_string().as_str())
}
