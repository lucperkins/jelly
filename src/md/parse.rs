use markdown_it::{
    parser::inline::Text,
    plugins::{
        cmark::{
            block::{heading::ATXHeading, paragraph::Paragraph},
            inline::{
                backticks::CodeInline,
                emphasis::{Em, Strong},
                link::Link,
                newline::{Hardbreak, Softbreak},
            },
        },
        extra::strikethrough::Strikethrough,
    },
    Node,
};

use crate::md::code::{add_code_block_rule, FancyCodeBlock};

// TODO: make this less kludgey
pub fn node_to_string(node: &Node) -> String {
    let mut text = String::new();
    for sub in node.children.iter() {
        println!("{:?}", sub);
        if let Some(txt) = sub.cast::<Text>() {
            text.push_str(&txt.content);
        } else if sub.is::<Paragraph>() {
            // Surround paragraphs with spaces to keep sentences from getting smushed together
            text.push_str(&format!(" {} ", &node_to_string(sub)));
        } else if let Some(code) = sub.cast::<FancyCodeBlock>() {
            text.push_str(&code.content);
        } else if sub.is::<Hardbreak>() || sub.is::<Softbreak>() {
            text.push(' ');
        } else if sub.is::<CodeInline>()
            || sub.is::<Link>()
            || sub.is::<Strong>()
            || sub.is::<Em>()
            || sub.is::<Strikethrough>()
            || sub.is::<ATXHeading>()
        {
            text.push_str(&node_to_string(sub));
        }
    }
    text.trim().to_owned()
}

pub fn render(ast: &Node) -> String {
    ast.render()
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

    // Replaces block::code::add
    add_code_block_rule(md);

    markdown_it::plugins::extra::add(md);

    md.parse(input)
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
