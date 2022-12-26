use std::vec::IntoIter;

use markdown_it::{plugins::cmark::block::heading::ATXHeading, Node};
use serde::Serialize;
use slug::slugify;

use crate::md::node_to_string;

#[derive(Debug, PartialEq, Serialize)]
pub struct TableOfContents(Vec<(Heading, TableOfContents)>);

impl TableOfContents {
    pub fn new(document: &Node) -> Self {
        toc_for_level(&document.children, 2)
    }

    #[cfg(test)]
    fn empty() -> Self {
        Self(vec![])
    }
}

fn toc_for_level(nodes: &[Node], level: u8) -> TableOfContents {
    let mut toc: Vec<(Heading, TableOfContents)> = Vec::new();
    for (idx, heading) in Headings(nodes) {
        if heading.level == level {
            toc.push((heading, toc_for_level(&nodes[idx..], level + 1)));
        }
    }

    TableOfContents(toc)
}

#[derive(Debug, PartialEq, Serialize)]
struct Heading {
    level: u8,
    text: String,
    slug: String,
}

impl Heading {
    fn new(level: u8, text: &str) -> Self {
        Self {
            level,
            text: String::from(text),
            slug: slugify(text),
        }
    }
}

struct Headings<'a>(&'a [Node]);

impl<'a> IntoIterator for Headings<'a> {
    type Item = (usize, Heading);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut headings: Vec<(usize, Heading)> = Vec::new();

        for (idx, node) in self.0.iter().enumerate() {
            if let Some(heading) = node.cast::<ATXHeading>() {
                headings.push((idx, Heading::new(heading.level, &node_to_string(node))));
            }
        }

        headings.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::md::{parse::ast, toc::Heading};
    use indoc::indoc;

    use super::TableOfContents;

    #[test]
    fn build_toc() {
        let md = indoc! {"
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
        "};

        let tree = ast(md);

        let toc = TableOfContents::new(&tree);

        assert_eq!(
            toc,
            TableOfContents(vec![
                (
                    Heading::new(2, "Now a heading 2"),
                    TableOfContents(vec![(
                        Heading::new(3, "Now a heading 3"),
                        TableOfContents(vec![(
                            Heading::new(4, "Let's go even deeper"),
                            TableOfContents::empty()
                        )])
                    )])
                ),
                (
                    Heading::new(2, "And now back to a heading 2"),
                    TableOfContents(vec![(
                        Heading::new(3, "Another heading 3"),
                        TableOfContents::empty()
                    )])
                ),
                (
                    Heading::new(2, "And yet another heading 2"),
                    TableOfContents::empty()
                ),
            ])
        );
    }
}
