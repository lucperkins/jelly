use std::vec::IntoIter;

use markdown_it::{plugins::cmark::block::heading::ATXHeading, Node};
use serde::Serialize;
use slug::slugify;

use super::node_to_string;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Heading {
    pub level: u8,
    text: String,
    slug: String,
}

impl Heading {
    pub fn new(level: u8, text: &str) -> Self {
        Self {
            level,
            text: String::from(text),
            slug: slugify(text),
        }
    }
}

pub struct Headings<'a>(pub &'a [Node]);

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
