use std::str::FromStr;

use crate::{ast1, ast2, ast3::Document};

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
    "#.trim();
    let ast = ast1::Document::from_str(content).unwrap();

    dbg!(ast);
    panic!()
}
