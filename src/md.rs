use comrak::{
    nodes::{AstNode, NodeCode, NodeValue},
    parse_document, Arena, ComrakOptions,
};

pub fn get_document_title(body: &str) -> Option<String> {
    let arena = Arena::new();
    let root = parse_document(&arena, body, &ComrakOptions::default());

    let mut num_headers = 0;

    for node in root.children() {
        if let NodeValue::Heading(heading) = node.data.borrow().value {
            num_headers += 1;

            if heading.level == 1 && num_headers == 1 {
                let mut text: Vec<u8> = Vec::new();
                get_header_text(node, &mut text);
                let h = String::from_utf8_lossy(&text).to_string();
                return Some(h);
            }
        }
    }

    None
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
