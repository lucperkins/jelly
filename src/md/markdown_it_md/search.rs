use markdown_it::Node;
use serde::Serialize;

use super::{headings::HeadingsWithTextAfter, parse::preamble};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SearchDocument {
    level: u8,
    page_title: String,
    title: String,
    content: String,
}

impl SearchDocument {
    pub fn new(level: u8, page_title: &str, title: &str, content: &str) -> Self {
        Self {
            level,
            page_title: String::from(page_title),
            title: String::from(title),
            content: String::from(content),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct SearchIndex(pub Vec<SearchDocument>);

pub fn build_search_index_for_page(page_title: &str, document: &Node) -> SearchIndex {
    let mut documents: Vec<SearchDocument> = Vec::new();

    documents.push(SearchDocument::new(
        1,
        page_title,
        page_title,
        &preamble(document),
    ));

    for (heading, s) in HeadingsWithTextAfter(&document.children) {
        documents.push(SearchDocument::new(
            heading.level,
            page_title,
            &heading.text,
            &s,
        ));
    }

    SearchIndex(documents)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::md::ast;

    use super::{build_search_index_for_page, SearchDocument, SearchIndex};

    #[test]
    fn search_index() {
        let cases: Vec<(&str, &str, SearchIndex)> = vec![
            (
                "First page",
                "",
                SearchIndex(vec![SearchDocument::new(1, "First page", "First page", "")]),
            ),
            (
                "Second page",
                indoc! {"
                    Some text.

                    ## h2

                    Some text content.

                    ### h3

                    And some more.

                    And some text from another paragraph.
                "},
                SearchIndex(vec![
                    SearchDocument::new(1, "Second page", "Second page", "Some text."),
                    SearchDocument::new(2, "Second page", "h2", "Some text content."),
                    SearchDocument::new(
                        3,
                        "Second page",
                        "h3",
                        "And some more. And some text from another paragraph.",
                    ),
                ]),
            ),
        ];

        for (page_title, md, expected_index) in cases {
            let tree = ast(md);
            let index = build_search_index_for_page(page_title, &tree);
            for (idx, doc) in index.0.iter().enumerate() {
                assert_eq!(doc, &expected_index.0[idx]);
            }
        }
    }
}
