use crate::error::ContentError;
use comrak::{
    nodes::{AstNode, NodeCode, NodeValue},
    parse_document, Arena, ComrakOptions,
};

pub fn get_document_title(body: &str) -> Result<Option<String>, ContentError> {
    let arena = Arena::new();
    let root = parse_document(&arena, body, &ComrakOptions::default());

    let mut num_headers = 0;

    for node in root.children() {
        let header = match node.data.clone().into_inner().value {
            NodeValue::Heading(c) => c,
            _ => continue,
        };

        num_headers += 1;

        if header.level == 1 && num_headers == 1 {
            let mut text: Vec<u8> = Vec::new();
            get_header_text(node, &mut text);
            return match String::from_utf8(text) {
                Ok(s) => Ok(Some(s)),
                Err(e) => Err(ContentError::Utf8(e)),
            };
        } else {
            continue;
        }
    }

    Ok(None)
}

fn get_header_text<'a>(node: &'a AstNode<'a>, output: &mut Vec<u8>) {
    match node.data.borrow().value {
        NodeValue::Text(ref literal) | NodeValue::Code(NodeCode { ref literal, .. }) => {
            output.extend_from_slice(literal)
        }
        NodeValue::LineBreak | NodeValue::SoftBreak => output.push(b' '),
        _ => {
            for n in node.children() {
                get_header_text(n, output);
            }
        }
    }
}
