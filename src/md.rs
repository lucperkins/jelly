use markdown_it::{parser::inline::Text, plugins::cmark::block::heading::ATXHeading, Node};

fn get_heading_text(node: &Node) -> Option<&String> {
    node.children[0].cast::<Text>().map(|t| &t.content)
}

pub fn get_document_title(body: &str) -> Option<String> {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    let ast = parser.parse(body);
    let mut num_headers = 0;

    for node in ast.children.iter() {
        if let Some(heading) = node.cast::<ATXHeading>() {
            num_headers += 1;

            if num_headers == 1 && heading.level == 1 {
                return get_heading_text(node).map(String::from);
            }
        }
    }

    None
}

pub fn render(md: &str) -> String {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);

    parser.parse(md).render()
}
