use super::{
    headings::FancyHeading,
    parse::{ast, node_to_string},
};

pub fn get_document_title(body: &str) -> Option<String> {
    let ast = ast(body);
    let mut num_headers = 0;

    for node in ast.children.iter() {
        if let Some(heading) = node.cast::<FancyHeading>() {
            num_headers += 1;

            if num_headers == 1 && heading.level == 1 {
                return Some(node_to_string(node));
            }
        }
    }

    None
}
