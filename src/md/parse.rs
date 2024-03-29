use markdown_it::{
    parser::inline::Text,
    plugins::{
        cmark::{
            block::paragraph::Paragraph,
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

use crate::md::{
    code::{add_code_block_rule, FancyCodeBlock},
    headings::add_heading_rule,
    image::add_image_rule,
};

use super::headings::FancyHeading;

pub(super) fn nodes_to_string(nodes: Vec<&Node>) -> String {
    nodes
        .iter()
        .map(|node| node_to_string(node))
        .collect::<Vec<String>>()
        .join(" ")
}

// Convert the text before any header to a string.
pub(super) fn preamble(node: &Node) -> String {
    let mut nodes: Vec<&Node> = Vec::new();

    for node in node.children.iter() {
        if let Some(heading) = node.cast::<FancyHeading>() {
            if heading.level == 1 {
                continue;
            } else {
                break;
            }
        } else {
            nodes.push(node);
        }
    }

    nodes_to_string(nodes)
}

// TODO: make this less kludgey
pub fn node_to_string(node: &Node) -> String {
    let mut pieces: Vec<String> = Vec::new();

    for sub in node.children.iter() {
        if let Some(txt) = sub.cast::<Text>() {
            pieces.push(txt.content.clone());
        } else if sub.is::<Paragraph>() {
            pieces.push(format!(" {} ", node_to_string(sub)));
        } else if let Some(code) = sub.cast::<FancyCodeBlock>() {
            pieces.push(code.content.clone());
        } else if sub.is::<CodeInline>()
            || sub.is::<Link>()
            || sub.is::<Strong>()
            || sub.is::<Em>()
            || sub.is::<Strikethrough>()
            || sub.is::<FancyHeading>()
        {
            pieces.push(node_to_string(sub));
        } else {
            pieces.push(node.render());
        }
    }

    pieces.join("").trim().to_owned()
}

pub fn render(ast: &Node) -> String {
    ast.render()
}

pub fn ast(input: &str) -> Node {
    use markdown_it::plugins::cmark::*;

    let md = &mut markdown_it::MarkdownIt::new();

    // cmark except code blocks
    inline::newline::add(md);
    inline::escape::add(md);
    inline::backticks::add(md);
    inline::emphasis::add(md);
    inline::link::add(md);
    // Replaces inline::image::add(md)
    add_image_rule(md);
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

    add_heading_rule(md);
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
