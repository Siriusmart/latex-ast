use std::str::FromStr;

use crate::{
    ast1::{Chunk, ChunkVariant, Command, Document, Scope, ScopeVariant},
    traits::Lines,
    InternalError,
};

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
fn lines() {
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

    assert_eq!(ast.lines(), 13)
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
        Chunk::new(1, ChunkVariant::Text("Hello world\n\n".to_string())).unwrap(),
        Chunk::new(
            3,
            ChunkVariant::Command(
                Command::new(
                    "textbf".to_string(),
                    vec![(
                        " ".to_string(),
                        Scope::new(
                            vec![
                                Chunk::new(1, ChunkVariant::Text("\n".to_string())).unwrap(),
                                Chunk::new(
                                    2,
                                    ChunkVariant::Command(
                                        Command::new(
                                            "container".to_string(),
                                            vec![
                                                (
                                                    String::new(),
                                                    Scope::new(
                                                        vec![Chunk::new(
                                                            1,
                                                            ChunkVariant::Text("123".to_string()),
                                                        )
                                                        .unwrap()],
                                                        ScopeVariant::Round,
                                                    )
                                                    .unwrap(),
                                                ),
                                                (
                                                    "\n".to_string(),
                                                    Scope::new(
                                                        vec![Chunk::new(
                                                            1,
                                                            ChunkVariant::Text("456".to_string()),
                                                        )
                                                        .unwrap()],
                                                        ScopeVariant::Square,
                                                    )
                                                    .unwrap(),
                                                ),
                                            ],
                                        )
                                        .unwrap(),
                                    ),
                                )
                                .unwrap(),
                                Chunk::new(3, ChunkVariant::Text("\ntext\n".to_string())).unwrap(),
                            ],
                            ScopeVariant::Curly,
                        )
                        .unwrap(),
                    )],
                )
                .unwrap(),
            ),
        )
        .unwrap(),
        Chunk::new(7, ChunkVariant::Text("\n\ntext\n\n".to_string())).unwrap(),
        Chunk::new(
            11,
            ChunkVariant::Scope(
                Scope::new(
                    vec![
                        Chunk::new(1, ChunkVariant::Text(" ".to_string())).unwrap(),
                        Chunk::new(
                            1,
                            ChunkVariant::Command(
                                Command::new("sin".to_string(), Vec::new()).unwrap(),
                            ),
                        )
                        .unwrap(),
                        Chunk::new(1, ChunkVariant::Text(" text ".to_string())).unwrap(),
                        Chunk::new(
                            1,
                            ChunkVariant::Command(
                                Command::new("sin".to_string(), Vec::new()).unwrap(),
                            ),
                        )
                        .unwrap(),
                        Chunk::new(1, ChunkVariant::Text(" ".to_string())).unwrap(),
                    ],
                    ScopeVariant::Curly,
                )
                .unwrap(),
            ),
        )
        .unwrap(),
        Chunk::new(11, ChunkVariant::Text("\n\nBye!".to_string())).unwrap(),
    ];

    assert_eq!(dbg!(ast.chunks()), dbg!(&expected));
    assert_eq!(dbg!(ast.to_string()), content.to_string());
}
