use markdown_it::Node;
use serde::Serialize;

use super::headings::{Heading, Headings};

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

    Headings(nodes)
        .into_iter()
        .filter(|(_, heading)| heading.level == level)
        .for_each(|(idx, heading)| {
            toc.push((heading, toc_for_level(&nodes[idx..], level + 1)));
        });

    TableOfContents(toc)
}

#[cfg(test)]
mod tests {
    use crate::md::{headings::Heading, parse::ast};
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
