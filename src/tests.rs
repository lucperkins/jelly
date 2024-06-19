#[cfg(test)]
pub(super) fn test_markdown_produces_expected_html(cases: Vec<(&str, &str)>) {
    use crate::md::{ast, render};

    for case in cases {
        let tree = ast(case.0);
        let html = render(&tree);
        assert_eq!(html, case.1);
    }
}
