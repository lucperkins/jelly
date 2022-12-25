use markdown_it::{plugins::cmark::block::heading::ATXHeading, Node};
use serde::Serialize;
use slug::slugify;

use crate::md::node_to_string;

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

#[derive(Debug, PartialEq, Serialize)]
pub struct TableOfContents(Vec<(Heading, TableOfContents)>);

impl TableOfContents {
    pub fn new(document: &Node, level: u8) -> Self {
        toc_for_level(&document.children, level)
    }

    #[cfg(test)]
    fn empty() -> Self {
        Self(vec![])
    }
}

fn toc_for_level(nodes: &[Node], level: u8) -> TableOfContents {
    let mut toc: Vec<(Heading, TableOfContents)> = Vec::new();

    for (idx, node) in nodes.iter().enumerate() {
        if let Some(h) = node.cast::<ATXHeading>() {
            if h.level == level {
                let text = &node_to_string(node);
                let heading = Heading::new(level, text);
                toc.push((heading, toc_for_level(&nodes[idx..], level + 1)));
            }
        }
    }

    TableOfContents(toc)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::md::{parse::ast, toc::Heading};

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

        let toc = TableOfContents::new(&tree, 2);

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
