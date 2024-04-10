use markdown_it::Node;

use crate::md::{code::add_code_block_rule, headings::add_heading_rule, image::add_image_rule};

use super::headings::FancyHeading;

pub(super) fn nodes_to_string(nodes: Vec<&Node>) -> String {
    nodes
        .iter()
        .map(|node| node.collect_text())
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

pub fn render(ast: &Node) -> String {
    ast.render()
}

pub fn ast(input: &str) -> Node {
    use markdown_it::plugins::cmark::{block, inline};

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

    // Replaces block::code::add(md)
    add_code_block_rule(md);
    block::fence::add(md);
    block::blockquote::add(md);
    block::hr::add(md);
    block::list::add(md);
    block::reference::add(md);
    // Replaces block::heading::add(md)
    add_heading_rule(md);
    block::lheading::add(md);
    block::paragraph::add(md);

    // Plugins
    use markdown_it::plugins::extra::{beautify_links, strikethrough, tables, typographer};

    strikethrough::add(md);
    beautify_links::add(md);
    // Disabled
    // linkify::add(md);
    tables::add(md);
    // Disabled
    // syntect::add(md);
    typographer::add(md);
    // Disabled (MAYBE: make this configurable?)
    // smartquotes::add(md);

    md.parse(input)
}
