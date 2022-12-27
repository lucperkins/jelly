use markdown_it::Node;
use serde::Serialize;

use super::headings::Heading;

#[derive(Debug, PartialEq, Serialize)]
struct TocEntry {
    heading: Heading,
    children: Option<Vec<TocEntry>>,
}

impl TocEntry {
    fn new(heading: Heading, children: Option<Vec<TocEntry>>) -> Self {
        Self { heading, children }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TableOfContents(Vec<TocEntry>);

impl TableOfContents {
    pub fn parse(document: &Node) -> Self {
        toc_for_level(&document.children, 2)
    }

    #[cfg(test)]
    fn empty() -> Self {
        Self(vec![])
    }
}

fn toc_for_level(nodes: &[Node], level: u8) -> TableOfContents {
    TableOfContents(vec![])
}

#[cfg(test)]
mod tests {
    use crate::md::{headings::Heading, parse::ast, toc::TocEntry};
    use indoc::indoc;

    use super::TableOfContents;

    #[test]
    fn build_toc() {
        // (input Markdown, expected TOC)
        let cases: Vec<(&str, TableOfContents)> = vec![
            ("", TableOfContents::empty()),
            (
                indoc! {"
                    Here is some text.

                    ## Now a heading 2

                    More text.

                    ### Now a heading 3

                    More text.

                    #### Let's go even deeper

                    Filler here.

                    ## And now back to a heading 2

                    More text.

                    ### Another heading 3

                    ## And yet another heading 2
                "},
                TableOfContents(vec![
                    TocEntry::new(
                        Heading::new(2, "Now a heading 2"),
                        Some(vec![TocEntry::new(
                            Heading::new(3, "Now a heading 3"),
                            Some(vec![TocEntry::new(
                                Heading::new(4, "Let's go even deeper"),
                                None,
                            )]),
                        )]),
                    ),
                    TocEntry::new(
                        Heading::new(2, "And now back to a heading 2"),
                        Some(vec![TocEntry::new(
                            Heading::new(3, "Another heading 3"),
                            None,
                        )]),
                    ),
                    TocEntry::new(Heading::new(2, "And yet another heading 2"), None),
                ]),
            ),
        ];

        for (md, expected_toc) in cases {
            let tree = ast(md);
            let toc = TableOfContents::parse(&tree);

            assert_eq!(toc, expected_toc);
        }
    }
}
