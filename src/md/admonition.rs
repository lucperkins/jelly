use markdown_it::{
    parser::block::{BlockRule, BlockState},
    MarkdownIt, Node, NodeValue, Renderer,
};

#[derive(Debug)]
enum AdmonitionKind {
    Danger,
    Info,
    Success,
    Warning,
}

impl Default for AdmonitionKind {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug)]
struct Admonition {
    kind: AdmonitionKind,
}

impl NodeValue for Admonition {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {}
}

struct AdmonitionScanner;

impl BlockRule for AdmonitionScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        None
    }
}

pub fn add_admonition_rule(md: &mut MarkdownIt) {
    md.block.add_rule::<AdmonitionScanner>();
}
