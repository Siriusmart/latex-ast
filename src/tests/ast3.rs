use std::str::FromStr;

use crate::ast3::Document;

#[test]
fn debug() {
    let content = r#"
\documentclass{article}\begin{document}$$test\$ ho$$\end{document}
    "#.trim();
    let ast = Document::from_str(content).unwrap();

    dbg!(ast);
    panic!()
}
