use markdown_it::{
    parser::inline::Text,
    plugins::{
        cmark::{
            block::heading::ATXHeading,
            inline::{
                backticks::CodeInline,
                emphasis::{Em, Strong},
                link::Link,
            },
        },
        extra::strikethrough::Strikethrough,
    },
    Node,
};

fn get_node_text(node: &Node) -> String {
    let mut text = String::new();
    for sub in node.children.iter() {
        if let Some(txt) = sub.cast::<Text>() {
            text.push_str(&txt.content);
        } else if sub.cast::<CodeInline>().is_some()
            || sub.cast::<Link>().is_some()
            || sub.cast::<Strong>().is_some()
            || sub.cast::<Em>().is_some()
            || sub.cast::<Strikethrough>().is_some()
        {
            text.push_str(&get_node_text(sub));
        } else {
            println!("{:?}", sub);
        }
    }
    println!("{:?}", text);
    text
}

pub fn get_document_title(body: &str) -> Option<String> {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);

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
