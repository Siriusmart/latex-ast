use std::str::FromStr;

use crate::ast1::{Chunk, ChunkVariant, Command, Document, Scope, ScopeVariant};

#[test]
fn unexpected_closing() {
    let content = r#"
Test

(
    (
        Hello
        (
            \badargs]
        )
    )
)
"#
    .trim();

    let ast = Document::from_str(content);

    assert_eq!(
        ast,
        Err(crate::Error::new(
            7,
            crate::ErrorType::UnexpectedClosing(ScopeVariant::Square)
        ))
    )
}

#[test]
fn unclosed_argument() {
    let content = r#"
test
(
    \hello
    test
    (
        test
        \hello[]
        \badargs[[[arg arg arg]]
    )
)
"#
    .trim();

    let ast = Document::from_str(content);

    assert_eq!(
        ast,
        Err(crate::Error::new(
            8,
            crate::ErrorType::UnclosedArgument(ScopeVariant::Square)
        ))
    )
}

#[test]
fn unclosed_scope() {
    let content = r#"
(
    \hello
    test
    \test{
        test
        [
    }
)
"#
    .trim();

    let ast = Document::from_str(content);

    assert_eq!(
        ast,
        Err(crate::Error::new(
            6,
            crate::ErrorType::UnclosedScope(ScopeVariant::Square)
        ))
    )
}

#[test]
fn basic() {
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

    let ast = Document::from_str(content).unwrap();

    let expected = vec![
        Chunk::new(1, ChunkVariant::Text("Hello world\n\n".to_string())),
        Chunk::new(
            3,
            ChunkVariant::Command(Command::new(
                "textbf".to_string(),
                vec![(
                    " ".to_string(),
                    Scope::new(
                        vec![
                            Chunk::new(1, ChunkVariant::Text("\n".to_string())),
                            Chunk::new(
                                2,
                                ChunkVariant::Command(Command::new(
                                    "container".to_string(),
                                    vec![
                                        (
                                            String::new(),
                                            Scope::new(
                                                vec![Chunk::new(
                                                    1,
                                                    ChunkVariant::Text("123".to_string()),
                                                )],
                                                ScopeVariant::Round,
                                            ),
                                        ),
                                        (
                                            "\n".to_string(),
                                            Scope::new(
                                                vec![Chunk::new(
                                                    1,
                                                    ChunkVariant::Text("456".to_string()),
                                                )],
                                                ScopeVariant::Square,
                                            ),
                                        ),
                                    ],
                                )),
                            ),
                            Chunk::new(3, ChunkVariant::Text("\ntext\n".to_string())),
                        ],
                        ScopeVariant::Curly,
                    ),
                )],
            )),
        ),
        Chunk::new(7, ChunkVariant::Text("\n\ntext\n\n".to_string())),
        Chunk::new(
            11,
            ChunkVariant::Scope(Scope::new(
                vec![
                    Chunk::new(1, ChunkVariant::Text(" ".to_string())),
                    Chunk::new(
                        1,
                        ChunkVariant::Command(Command::new("sin".to_string(), Vec::new())),
                    ),
                    Chunk::new(1, ChunkVariant::Text(" text ".to_string())),
                    Chunk::new(
                        1,
                        ChunkVariant::Command(Command::new("sin".to_string(), Vec::new())),
                    ),
                    Chunk::new(1, ChunkVariant::Text(" ".to_string())),
                ],
                ScopeVariant::Curly,
            )),
        ),
        Chunk::new(11, ChunkVariant::Text("\n\nBye!".to_string())),
    ];

    assert_eq!(dbg!(ast.chunks()), dbg!(&expected));
    assert_eq!(dbg!(ast.to_string()), content.to_string());
}
