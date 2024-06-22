use std::str::FromStr;

use crate::{
    ast1,
    ast2::{Chunk, ChunkVariant, Command, Document, Environment, Scope, ScopeVariant},
    traits::Lines,
    InternalError,
};

#[test]
fn no_environment_label() {
    let content = r#"
\usepackage{amsmath}

\begin{document}
    Hello
    \begin
        \item test
    \end{itemize}
\end{document}
        "#
    .trim();

    let ast = Document::from_str(content);

    assert_eq!(
        ast,
        Err(crate::Error::new(5, crate::ErrorType::NoEnvironmentLabel))
    )
}

#[test]
fn unexpected_end() {
    let content = r#"
\usepackage{amsmath}

\begin{document}
    Hello
        \item test
    \end{itemize}
\end{document}
        "#
    .trim();

    let ast = Document::from_str(content);

    assert_eq!(
        ast,
        Err(crate::Error::new(
            6,
            crate::ErrorType::UnexpectedEnd("itemize".to_string())
        ))
    )
}

#[test]
fn too_many_args_end() {
    let content = r#"
\usepackage{amsmath}

\begin{document}
    Hello
    \begin{itemize}
        \item test
    \end{itemize}{boom}
\end{document}
        "#
    .trim();

    let ast = Document::from_str(content);

    assert_eq!(
        ast,
        Err(crate::Error::new(7, crate::ErrorType::TooManyArgsEnd))
    )
}

#[test]
fn construct() {
    let mut ast = Document::default();

    ast.push(ChunkVariant::Text("hello world\n".to_string()))
        .unwrap();
    ast.push(ChunkVariant::Command(
        Command::new("command".to_string(), Vec::new()).unwrap(),
    ))
    .unwrap();
    ast.push(ChunkVariant::Text("continuation\n".to_string()))
        .unwrap();
    ast.push(ChunkVariant::Text(
        "text should be merged into one chunk".to_string(),
    ))
    .unwrap();

    assert_eq!(
        ast.chunks_owned(),
        vec![
            Chunk::new(1, ChunkVariant::Text("hello world\n".to_string())).unwrap(),
            Chunk::new(
                2,
                ChunkVariant::Command(Command::new("command".to_string(), Vec::new()).unwrap())
            )
            .unwrap(),
            Chunk::new(
                2,
                ChunkVariant::Text(
                    "continuation\ntext should be merged into one chunk".to_string()
                )
            )
            .unwrap()
        ]
    )
}

#[test]
fn incorrect_line() {
    let content = r#"
Hello world

\textbf {
\container(123)
[456]
text
}

text

{ \sin text \sin }

Bye!
"#
    .trim();

    let mut ast = Document::from_str(content).unwrap();

    let new = Chunk::new(11, ChunkVariant::Text("hello".to_string())).unwrap();

    assert_eq!(
        Err(InternalError::IncorrectChunkLineNumber {
            expected: 13,
            got: 11
        }),
        ast.push_chunk(new)
    );
}

#[test]
fn unsanitised_char() {
    let mut ast = Document::default();

    assert_eq!(
        Err(InternalError::UnsanitisedCharInString('\\')),
        ast.push(ChunkVariant::Text("hello\\world".to_string()))
    );
}

#[test]
fn lines() {
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

    let ast = Document::from_str(content).unwrap();

    assert_eq!(ast.lines(), 8)
}

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
                ChunkVariant::Command(
                    Command::new(
                        "usepackage".to_string(),
                        vec![(
                            "".to_string(),
                            Scope::new(
                                vec![Chunk::new(1, ChunkVariant::Text("amsmath".to_string()))
                                    .unwrap()],
                                ScopeVariant::Curly
                            )
                            .unwrap()
                        )]
                    )
                    .unwrap()
                )
            )
            .unwrap(),
            Chunk::new(1, ChunkVariant::Text("\n\n".to_string())).unwrap(),
            Chunk::new(
                3,
                ChunkVariant::Environment(
                    Environment::new(
                        "document".to_string(),
                        Vec::new(),
                        vec![
                            Chunk::new(1, ChunkVariant::Text("\n    Hello\n    ".to_string()))
                                .unwrap(),
                            Chunk::new(
                                3,
                                ChunkVariant::Environment(
                                    Environment::new(
                                        "itemize".to_string(),
                                        Vec::new(),
                                        vec![
                                            Chunk::new(
                                                1,
                                                ChunkVariant::Text("\n        ".to_string())
                                            )
                                            .unwrap(),
                                            Chunk::new(
                                                2,
                                                ChunkVariant::Command(
                                                    Command::new("item".to_string(), Vec::new())
                                                        .unwrap()
                                                )
                                            )
                                            .unwrap(),
                                            Chunk::new(
                                                2,
                                                ChunkVariant::Text(" test\n    ".to_string())
                                            )
                                            .unwrap()
                                        ],
                                        String::new(),
                                        String::new()
                                    )
                                    .unwrap()
                                )
                            )
                            .unwrap(),
                            Chunk::new(5, ChunkVariant::Text("\n".to_string())).unwrap()
                        ],
                        String::new(),
                        String::new()
                    )
                    .unwrap()
                )
            )
            .unwrap(),
        ])
        .unwrap())
    );

    assert_eq!(dbg!(one), dbg!(ast1::Document::from(two.clone())));
    assert_eq!(content, two.to_string().as_str())
}
