use markdown_it::{parser::inline::Text, plugins::cmark::block::heading::ATXHeading};

pub fn get_document_title(body: &str) -> Option<String> {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    let ast = parser.parse(body);
    let mut num_headers = 0;

    let mut title: Option<&String> = None;

    for node in ast.children.iter() {
        if let Some(heading) = node.cast::<ATXHeading>() {
            num_headers += 1;

            if num_headers == 1 && heading.level == 1 {
                if let Some(text) = node.children[0].cast::<Text>() {
                    title = Some(&text.content);
                }
            }
        }
    }

    title.map(String::from)
}
