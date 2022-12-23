use markdown_it::{plugins::cmark::block::heading::ATXHeading, Node};
use serde::Serialize;
use slug::slugify;

use crate::md::{ast, node_to_string};

#[derive(Debug, Serialize)]
struct Heading {
    level: u8,
    text: String,
    slug: String,
}

#[derive(Debug, Serialize)]
pub struct TableOfContents(Vec<(Heading, TableOfContents)>);

impl TableOfContents {
    pub fn new(document: &Node, level: u8) -> Self {
        toc_for_level(&document.children, level)
    }
}

fn toc_for_level(nodes: &[Node], level: u8) -> TableOfContents {
    let mut toc: Vec<(Heading, TableOfContents)> = Vec::new();

    for node in nodes {
        if let Some(h) = node.cast::<ATXHeading>() {
            //println!("{:?}", h);
            if h.level == level {
                let text = &node_to_string(node);
                let slug = slugify(text);
                let heading = Heading {
                    level,
                    text: String::from(text),
                    slug,
                };

                toc.push((heading, toc_for_level(nodes, level + 1)));
            }
        }
    }

    TableOfContents(toc)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::md::ast;

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

            ## And now back to a heading 2

            More text.

            ## And yet another heading 2
        "};

        let tree = ast(md);

        let toc = TableOfContents::new(&tree, 2);

        println!("{:?}", toc.0[0]);
    }
}
