use markdown_it::{
    parser::{core::CoreRule, inline::Text},
    plugins::{
        cmark::{
            block::{code::CodeBlock, fence::CodeFence, heading::ATXHeading, paragraph::Paragraph},
            inline::{
                backticks::CodeInline,
                emphasis::{Em, Strong},
                link::Link,
                newline::{Hardbreak, Softbreak},
            },
        },
        extra::strikethrough::Strikethrough,
    },
    MarkdownIt, Node, NodeValue, Renderer,
};

use crate::highlight::Highlighter;

// TODO: make this less kludgey
fn node_to_string(node: &Node) -> String {
    let mut text = String::new();
    for sub in node.children.iter() {
        println!("{:?}", sub);
        if let Some(txt) = sub.cast::<Text>() {
            text.push_str(&txt.content);
        } else if let Some(code) = sub.cast::<FancyCodeBlock>() {
            text.push_str(&code.content);
        } else if sub.is::<CodeInline>()
            || sub.is::<Link>()
            || sub.is::<Strong>()
            || sub.is::<Em>()
            || sub.is::<Strikethrough>()
            || sub.is::<ATXHeading>()
        {
            text.push_str(&node_to_string(sub));
        } else if sub.is::<Hardbreak>() || sub.is::<Softbreak>() {
            text.push(' ');
        } else if sub.is::<Paragraph>() {
            text.push_str(&format!(" {} ", &node_to_string(sub)));
        } else if let Some(h) = sub.children.get(0) {
            if let Some(t) = h.cast::<Text>() {
                text.push_str(&t.content);
            }
        }
    }
    text.trim().to_owned()
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

pub fn ast(input: &str) -> Node {
    let md = &mut markdown_it::MarkdownIt::new();

    // cmark except code blocks
    use markdown_it::plugins::cmark::*;
    inline::newline::add(md);
    inline::escape::add(md);
    inline::backticks::add(md);
    inline::emphasis::add(md);
    inline::link::add(md);
    inline::image::add(md);
    inline::autolink::add(md);
    inline::entity::add(md);

    block::fence::add(md);
    block::blockquote::add(md);
    block::hr::add(md);
    block::list::add(md);
    block::reference::add(md);
    block::heading::add(md);
    block::lheading::add(md);
    block::paragraph::add(md);

    md.add_rule::<FancyCodeBlocks>();

    markdown_it::plugins::extra::add(md);

    md.parse(input)
}

#[derive(Debug)]
struct FancyCodeBlock {
    meta: Option<String>,
    content: String,
}

impl NodeValue for FancyCodeBlock {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        let default_lang = String::from("text");
        let lang = self.meta.as_ref().unwrap_or(&default_lang);
        let language = format!("language-{}", lang);
        let pre_attrs = vec![("class", language)];

        let higlighter = Highlighter::default();

        let code = match higlighter.highlight(lang, &self.content) {
            Ok(html) => html,
            Err(e) => e.to_string(),
        };

        fmt.cr();
        fmt.open("pre", &pre_attrs);
        fmt.open("code", &[]);
        fmt.cr();
        fmt.text_raw(&code);
        fmt.cr();
        fmt.close("code");
        fmt.close("pre");
        fmt.cr()
    }
}

struct FancyCodeBlocks;

impl CoreRule for FancyCodeBlocks {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_post_mut(|node, _| {
            let mut meta: Option<&String> = None;
            let mut content: Option<&String> = None;

            if let Some(code) = node.cast::<CodeBlock>() {
                content = Some(&code.content);
            }
            if let Some(code) = node.cast::<CodeFence>() {
                meta = Some(&code.info);
                content = Some(&code.content);
            }

            if let Some(content) = content {
                node.replace(FancyCodeBlock {
                    meta: meta.map(String::from),
                    content: String::from(content),
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{ast, node_to_string};

    #[test]
    fn node_to_string_fn() {
        let cases: Vec<(&str, &str)> = vec![
            ("", ""),
            (
                r#"## Some `code` and some **bold** and some *italics*"#,
                "Some code and some bold and some italics",
            ),
            (
                r#"A link to [Google](https://google.com)"#,
                "A link to Google",
            ),
            (
                indoc! {"
                    Some normal text.

                    ## And then a header
                "},
                "Some normal text. And then a header",
            ),
            (
                indoc! {"
                    Some text.

                    ## And then a header with `code`
                "},
                "Some text. And then a header with code",
            ),
            (
                indoc! {"
                    Some text plus **bold**.

                    ```python
                    x = 5
                    ```
                "},
                "Some text plus bold. x = 5",
            ),
        ];

        for (md, expected) in cases {
            let tree = ast(md);
            let output = &node_to_string(&tree);
            assert_eq!(output, expected);
        }
    }
}
