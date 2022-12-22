use markdown_it::{
    parser::{block::LineOffset, inline::Text},
    plugins::cmark::{
        block::heading::ATXHeading,
        inline::{backticks::CodeInline, link::Link, newline::Softbreak},
    },
    Node,
};

fn get_node_text(node: &Node) -> String {
    let mut text = String::new();
    for sub in node.children.iter() {
        if let Some(txt) = sub.cast::<Text>() {
            text.push_str(&txt.content);
        } else if sub.cast::<CodeInline>().is_some() {
            text.push_str(&get_node_text(&sub));
        } else if sub.cast::<Link>().is_some() {
            text.push_str(&get_node_text(&sub));
        }
    }
    text
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
                return Some(get_node_text(node));
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
