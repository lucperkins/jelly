use std::vec::IntoIter;

use markdown_it::{
    parser::{
        block::{BlockRule, BlockState},
        inline::InlineRoot,
    },
    MarkdownIt, Node, NodeValue, Renderer,
};
use serde::Serialize;
use slug::slugify;

use super::node_to_string;

#[derive(Debug)]
pub struct FancyHeading {
    pub level: u8,
}

fn h_attrs<'a>(slug: &str) -> Vec<(&'a str, String)> {
    vec![
        ("id", String::from(slug)),
        ("class", String::from("heading")),
    ]
}

fn a_attrs<'a>(slug: &str) -> Vec<(&'a str, String)> {
    vec![("href", format!("#{}", slug))]
}

impl NodeValue for FancyHeading {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        static TAG: [&str; 6] = ["h1", "h2", "h3", "h4", "h5", "h6"];
        debug_assert!(self.level >= 1 && self.level <= 6);

        // Add slug to attributes
        let slug = slugify(node_to_string(node));
        let h_attrs = h_attrs(&slug);
        let a_attrs = a_attrs(&slug);

        fmt.cr();
        fmt.open(TAG[self.level as usize - 1], &h_attrs);
        fmt.open("a", &a_attrs);
        fmt.contents(&node.children);
        fmt.close("a");
        fmt.close(TAG[self.level as usize - 1]);
        fmt.cr();
    }
}

pub struct FancyHeadingsRule;
impl BlockRule for FancyHeadingsRule {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        // if it's indented more than 3 spaces, it should be a code block
        if state.line_indent(state.line) >= 4 {
            return None;
        }

        let line = state.get_line(state.line);

        if let Some('#') = line.chars().next() {
        } else {
            return None;
        }

        let text_pos;

        // count heading level
        let mut level = 0u8;
        let mut chars = line.char_indices();
        loop {
            match chars.next() {
                Some((_, '#')) => {
                    level += 1;
                    if level > 6 {
                        return None;
                    }
                }
                Some((x, ' ' | '\t')) => {
                    text_pos = x;
                    break;
                }
                None => {
                    text_pos = level as usize;
                    break;
                }
                Some(_) => return None,
            }
        }

        // Let's cut tails like '    ###  ' from the end of string

        let mut chars_back = chars.rev().peekable();
        while let Some((_, ' ' | '\t')) = chars_back.peek() {
            chars_back.next();
        }
        while let Some((_, '#')) = chars_back.peek() {
            chars_back.next();
        }

        let text_max = match chars_back.next() {
            // ## foo ##
            Some((last_pos, ' ' | '\t')) => last_pos + 1,
            // ## foo##
            Some(_) => line.len(),
            // ## ## (already consumed the space)
            None => text_pos,
        };

        let content = line[text_pos..text_max].to_owned();
        let mapping = vec![(0, state.line_offsets[state.line].first_nonspace + text_pos)];

        let mut node = Node::new(FancyHeading { level });
        node.children
            .push(Node::new(InlineRoot::new(content, mapping)));
        Some((node, 1))
    }
}

pub fn add_heading_rule(md: &mut MarkdownIt) {
    md.block.add_rule::<FancyHeadingsRule>();
}

pub struct Headings<'a>(pub &'a [Node]);
pub struct HeadingsWithIdx<'a>(pub &'a [Node]);

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    slug: String,
}

impl Heading {
    pub fn new(level: u8, text: &str) -> Self {
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
            if let Some(heading) = node.cast::<FancyHeading>() {
                if heading.level > 1 {
                    headings.push(Heading::new(heading.level, &node_to_string(node)));
                }
            }
        }

        headings.into_iter()
    }
}

impl<'a> IntoIterator for HeadingsWithIdx<'a> {
    type Item = (usize, Heading);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut headings: Vec<(usize, Heading)> = Vec::new();

        for (idx, node) in self.0.iter().enumerate() {
            if let Some(heading) = node.cast::<FancyHeading>() {
                if heading.level > 1 {
                    headings.push((idx, Heading::new(heading.level, &node_to_string(node))));
                }
            }
        }

        headings.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::md::{ast, render};

    use super::{Heading, Headings};

    #[test]
    fn fancy_headings() {
        let cases: Vec<(&str, &str)> = vec![(
            "## Hello world",
            "<h2 id=\"hello-world\" class=\"heading\"><a href=\"#hello-world\">Hello world</a></h2>\n",
        ), (
            "### A heading with some `code`",
            "<h3 id=\"a-heading-with-some-code\" class=\"heading\"><a href=\"#a-heading-with-some-code\">A heading with some <code>code</code></a></h3>\n",
        )];

        for (md, expected_html) in cases {
            let html = render(&ast(md));
            assert_eq!(html, expected_html);
        }
    }

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
                    # h1 gets ignored

                    ## h2 with `bold`

                    ### h3 with `italics`

                    # h1 gets ignored

                    ## h2 has a `code sample`

                    #### h4

                    ##### h5

                    ## h2

                    ###### h6 has a [link with `code in it`](https://example.com)

                    ####### This won't show up
                "},
                vec![
                    Heading::new(2, "h2 with bold"),
                    Heading::new(3, "h3 with italics"),
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
