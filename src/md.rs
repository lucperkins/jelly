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

fn node_to_string(node: &Node) -> String {
    let mut text = String::new();
    for sub in node.children.iter() {
        if let Some(txt) = sub.cast::<Text>() {
            text.push_str(&txt.content);
        } else if sub.is::<CodeInline>()
            || sub.is::<Link>()
            || sub.is::<Strong>()
            || sub.is::<Em>()
            || sub.is::<Strikethrough>()
        {
            text.push_str(&node_to_string(sub));
        } else if let Some(n) = sub.children.get(0) {
            if let Some(t) = n.cast::<Text>() {
                text.push_str(&t.content);
            }
        }
    }
    text
}

pub fn get_document_title(body: &str) -> Option<String> {
    let ast = ast(body);
    let mut num_headers = 0;

    for node in ast.children.iter() {
        if let Some(heading) = node.cast::<ATXHeading>() {
            num_headers += 1;

            if num_headers == 1 && heading.level == 1 {
                return Some(node_to_string(node));
            }
        }
    }

    None
}

pub fn render(md: &str) -> String {
    ast(md).render()
}

pub fn ast(md: &str) -> Node {
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);

    parser.parse(md)
}
