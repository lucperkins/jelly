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

impl From<&str> for AdmonitionKind {
    fn from(s: &str) -> Self {
        use AdmonitionKind::*;

        match s {
            "danger" => Danger,
            "info" => Info,
            "success" => Success,
            "warning" => Warning,
            _ => Self::default(),
        }
    }
}

impl ToString for AdmonitionKind {
    fn to_string(&self) -> String {
        use AdmonitionKind::*;

        String::from(match *self {
            Danger => "danger",
            Info => "info",
            Success => "success",
            Warning => "warning",
        })
    }
}

#[derive(Debug)]
struct Admonition {
    kind: AdmonitionKind,
}

impl NodeValue for Admonition {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let div_attrs = &[("admonition-type", self.kind.to_string())];

        fmt.cr();
        fmt.open("div", div_attrs);
        fmt.contents(&node.children);
        fmt.close("div");
        fmt.cr();
    }
}

struct AdmonitionScanner;

impl BlockRule for AdmonitionScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let line = state.get_line(state.line);
        let last_line = state.get_line(state.line_max - 1);

        if line.starts_with("::") && last_line == "::" {
            Some((
                Node::new(Admonition {
                    kind: AdmonitionKind::Info,
                }),
                1,
            ))
        } else {
            None
        }
    }
}

pub fn add_admonition_rule(md: &mut MarkdownIt) {
    md.block.add_rule::<AdmonitionScanner>();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::md::{ast, render};

    #[test]
    fn render_admonitions() {
        let cases: Vec<(&str, &str)> = vec![(
            indoc! {"
            ::alert
            This doesn't work.

            ::alert
            But this does.
            ::
        "},
            indoc! {"<p>::alert\nThis doesn't work.</p>\n"},
        )];

        for (input, expected_html) in cases {
            let tree = ast(input);
            let html = render(&tree);
            assert_eq!(expected_html, &html);
        }
    }
}
