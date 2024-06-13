use std::str::FromStr;

use crate::ast1;

#[test]
fn debug() {
    let content = r#"
\documentclass{article}

\begin{document}
test
(
    \test]
)
\end{document}
    "#
    .trim();
    let ast = ast1::Document::from_str(content).unwrap();

    dbg!(ast);
    panic!()
}
