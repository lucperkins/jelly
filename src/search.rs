use markdown_it::plugins::cmark::block::heading::ATXHeading;
use serde::Serialize;

use crate::{
    md::{ast, node_to_string},
    page::Page,
};
#[derive(Debug, Serialize)]
pub struct Document {
    title: String,
    content: String,
}

pub struct Index;

pub fn build_search_index_for_page(page: &Page) -> Index {
    let ast = ast(&page.body);
    let nodes = &ast.children;

    let mut documents: Vec<Document> = Vec::new();

    for (idx, node) in nodes.iter().enumerate() {
        if node.is::<ATXHeading>() {
            let title = node_to_string(node);

            let mut here = idx;
            let mut content = String::new();

            loop {
                here += 1;

                if here == nodes.len() {
                    break;
                }

                if let Some(n) = &nodes.get(here) {
                    if n.is::<ATXHeading>() {
                        break;
                    }

                    content.push_str(&node_to_string(n));
                }
            }

            let document = Document { title, content };
            documents.push(document);
        }
    }

    Index
}
