use markdown_it::{plugins::cmark::block::heading::ATXHeading, Node};
use serde::Serialize;

use crate::md::node_to_string;

use super::headings::HeadingsWithIdx;

#[derive(Debug, Eq, PartialEq, Serialize)]
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

#[cfg(test)]
impl SearchIndex {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

pub fn build_search_index_for_page(page_title: &str, document: &Node) -> SearchIndex {
    let nodes = &document.children;

    let mut documents: Vec<SearchDocument> = Vec::new();

    for (idx, heading) in HeadingsWithIdx(nodes) {
        let mut here = idx;
        let mut pieces: Vec<String> = Vec::new();

        loop {
            here += 1;

            if here == nodes.len() {
                break;
            }

            if let Some(n) = &nodes.get(here) {
                if n.is::<ATXHeading>() {
                    break;
                }

                pieces.push(node_to_string(n));
            }
        }

        let content = pieces.join(" ");
        let final_content = content.trim();
        documents.push(SearchDocument::new(
            heading.level,
            page_title,
            &heading.text,
            final_content,
        ))
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
            ("First page", "", SearchIndex::empty()),
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
