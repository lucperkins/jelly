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

use super::parse::nodes_to_string;

#[derive(Debug)]
pub(super) struct FancyHeading {
    pub(super) level: u8,
}

fn h_attrs<'a>(slug: &str) -> Vec<(&'a str, String)> {
    vec![
        ("id", String::from(slug)),
        // For AlpineJS
        ("x-data", String::from("{ open: false }")),
        ("@mouseover", String::from("open = true")),
        ("@mouseout", String::from("open = false")),
    ]
}

fn a_attrs<'a>(slug: &str) -> Vec<(&'a str, String)> {
    vec![
        ("href", format!("#{}", slug)),
        ("x-show", String::from("open")),
        ("x-transition.duration.200ms", String::from("")),
        ("class", String::from("ml-2 text-primary not-prose")),
    ]
}

impl NodeValue for FancyHeading {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        if self.level == 1 {
            return;
        }

        static TAG: [&str; 5] = ["h2", "h3", "h4", "h5", "h6"];
        debug_assert!(self.level >= 2 && self.level <= 6);

        // Add slug to attributes
        let slug = slugify(node.collect_text());
        let h_attrs = h_attrs(&slug);
        let a_attrs = a_attrs(&slug);

        fmt.cr();
        fmt.open(TAG[self.level as usize - 2], &h_attrs);
        fmt.contents(&node.children);
        fmt.open("a", &a_attrs);
        fmt.text("#");
        fmt.close("a");
        fmt.close(TAG[self.level as usize - 2]);
        fmt.cr();
    }
}

struct FancyHeadingsRule;

impl BlockRule for FancyHeadingsRule {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        // if it's indented more than 3 spaces, it should be a code block
        if state.line_indent(state.line) >= 4 {
            return None;
        }

        let line = state.get_line(state.line);
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

pub(super) fn add_heading_rule(md: &mut MarkdownIt) {
    md.block.add_rule::<FancyHeadingsRule>();
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub(super) struct Heading {
    pub(super) level: u8,
    pub(super) text: String,
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

pub(super) struct Headings<'a>(pub(super) &'a [Node]);

impl<'a> IntoIterator for Headings<'a> {
    type Item = Heading;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut headings: Vec<Heading> = Vec::new();

        for node in self.0.iter() {
            if let Some(heading) = node.cast::<FancyHeading>() {
                if heading.level > 1 {
                    headings.push(Heading::new(heading.level, &node.collect_text()));
                }
            }
        }

        headings.into_iter()
    }
}

struct HeadingsWithIdx<'a>(&'a [Node]);

impl<'a> IntoIterator for HeadingsWithIdx<'a> {
    type Item = (usize, Heading);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut headings: Vec<(usize, Heading)> = Vec::new();

        for (idx, node) in self.0.iter().enumerate() {
            if let Some(heading) = node.cast::<FancyHeading>() {
                if heading.level > 1 {
                    headings.push((idx, Heading::new(heading.level, &node.collect_text())));
                }
            }
        }

        headings.into_iter()
    }
}

pub(super) struct HeadingsWithTextAfter<'a>(pub(super) &'a [Node]);

impl<'a> IntoIterator for HeadingsWithTextAfter<'a> {
    type Item = (Heading, String);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut documents: Vec<(Heading, String)> = Vec::new();

        for (idx, node) in self.0.iter().enumerate() {
            if let Some(heading) = node.cast::<FancyHeading>() {
                if heading.level > 1 {
                    let next = idx + 1;

                    let heading = Heading::new(heading.level, &node.collect_text());

                    match self.0.get(next) {
                        Some(next_node) => {
                            if next_node.cast::<FancyHeading>().is_some() {
                                documents.push((heading, String::from("")));
                            } else {
                                let mut n = next;
                                let mut nodes: Vec<&Node> = Vec::new();

                                while let Some(inner) = self.0.get(n) {
                                    if inner.cast::<FancyHeading>().is_some() {
                                        break;
                                    } else {
                                        nodes.push(inner);
                                    }

                                    n += 1;
                                }

                                documents.push((heading, nodes_to_string(nodes)));
                            }
                        }
                        None => {
                            documents.push((heading, String::from("")));
                        }
                    }
                }
            }
        }

        documents.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{md::ast, tests::test_markdown_produces_expected_html};

    use super::{Heading, Headings, HeadingsWithTextAfter};

    #[test]
    fn search_index() {
        let cases: Vec<(&str, Vec<(&str, &str)>)> = vec![(
            indoc! {"
                ## Heading 1

                Here is some text.

                ### Heading 2

                ## Heading 3

                Here is some other text.

                ### Heading 4

                **Bold**, _italics_, and `code`.

                Some other info.
            "},
            vec![
                ("Heading 1", "Here is some text."),
                ("Heading 2", ""),
                ("Heading 3", "Here is some other text."),
                ("Heading 4", "Bold, italics, and code. Some other info."),
            ],
        )];

        for (md, documents) in cases {
            let tree = ast(md);
            let headings: Vec<(Heading, String)> =
                HeadingsWithTextAfter(&tree.children).into_iter().collect();
            let docs: Vec<(&str, &str)> = headings
                .iter()
                .map(|(heading, s)| (heading.text.as_str(), s.as_str()))
                .collect();

            assert_eq!(docs, documents);
        }
    }

    #[test]
    fn fancy_headings() {
        let cases: Vec<(&str, &str)> = vec![
            (
                "## Hello world",
                "<h2 id=\"hello-world\" x-data=\"{ open: false }\" @mouseover=\"open = true\" @mouseout=\"open = false\">Hello world<a href=\"#hello-world\" x-show=\"open\" x-transition.duration.200ms=\"\" class=\"ml-2 text-primary not-prose\">#</a></h2>\n",
            ),
            (
                "### A heading with some `code`",
                "<h3 id=\"a-heading-with-some-code\" x-data=\"{ open: false }\" @mouseover=\"open = true\" @mouseout=\"open = false\">A heading with some <code>code</code><a href=\"#a-heading-with-some-code\" x-show=\"open\" x-transition.duration.200ms=\"\" class=\"ml-2 text-primary not-prose\">#</a></h3>\n",
            ),
        ];

        test_markdown_produces_expected_html(cases);
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
