use std::vec::IntoIter;

use markdown_it::{plugins::cmark::block::heading::ATXHeading, Node};
use slug::slugify;

use super::node_to_string;

struct Headings<'a>(&'a [Node]);

#[derive(Debug, PartialEq)]
struct Heading {
    level: u8,
    text: String,
    slug: String,
}

impl Heading {
    fn new(level: u8, text: &str) -> Self {
        let slug = slugify(text);
        Self {
            level,
            text: String::from(text),
            slug,
        }
    }
}

impl<'a> IntoIterator for Headings<'a> {
    type Item = Heading;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut headings: Vec<Heading> = Vec::new();

        for node in self.0.iter() {
            if let Some(heading) = node.cast::<ATXHeading>() {
                headings.push(Heading::new(heading.level, &node_to_string(node)));
            }
        }

        headings.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::md::ast;

    use super::{Heading, Headings};

    #[test]
    fn md_to_headings() {
        let cases: Vec<(&str, Vec<Heading>)> = vec![
            (indoc! {""}, vec![]),
            (
                indoc! {"
                    Some text.

                    ## Heading 2

                    Other text.

                    ### Heading 3

                    More text.

                    ## Back to heading 2
                "},
                vec![
                    Heading::new(2, "Heading 2"),
                    Heading::new(3, "Heading 3"),
                    Heading::new(2, "Back to heading 2"),
                ],
            ),
            (
                indoc! {"
                    # h1

                    ## h2 with `bold`

                    ### h3 with `italics`

                    # h1

                    ## h2 has a `code sample`

                    #### h4

                    ##### h5

                    ## h2

                    ###### h6 has a [link with `code in it`](https://example.com)

                    ####### This won't show up
                "},
                vec![
                    Heading::new(1, "h1"),
                    Heading::new(2, "h2 with bold"),
                    Heading::new(3, "h3 with italics"),
                    Heading::new(1, "h1"),
                    Heading::new(2, "h2 has a code sample"),
                    Heading::new(4, "h4"),
                    Heading::new(5, "h5"),
                    Heading::new(2, "h2"),
                    Heading::new(6, "h6 has a link with code in it"),
                ],
            ),
        ];

        for (md, expected_headings) in cases {
            let tree = ast(md);
            let headings: Vec<Heading> = Headings(&tree.children).into_iter().collect();

            for (idx, heading) in headings.iter().enumerate() {
                assert_eq!(heading, &expected_headings[idx]);
            }
        }
    }
}
