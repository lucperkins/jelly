use markdown_it::{
    parser::core::CoreRule,
    plugins::cmark::block::{code::CodeBlock, fence::CodeFence},
    MarkdownIt, Node, NodeValue, Renderer,
};

use super::highlight::Highlighter;

#[derive(Debug, Default)]
struct Metadata {
    language: Option<String>,
    show_line_numbers: bool,
    file: Option<String>,
}

impl Metadata {
    fn parse(s: &str) -> Self {
        let mut metadata = Metadata {
            language: None,
            show_line_numbers: false,
            file: None,
        };
        let mut parts = s.split_whitespace();

        if let Some(part) = parts.next() {
            metadata.language = Some(String::from(part));
        }

        for part in parts {
            if part == "showLineNumbers" {
                metadata.show_line_numbers = true;
            }

            if let Some(file) = {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 && parts[0].trim() == "file" {
                    let filename = parts[1].trim().trim_matches('"');
                    Some(filename.to_string())
                } else {
                    None
                }
            } {
                metadata.file = Some(file);
            }
        }
        metadata
    }
}

#[derive(Debug)]
struct FancyCodeBlock {
    meta: Metadata,
    content: String,
}

impl NodeValue for FancyCodeBlock {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        // TODO: make default language configurable
        let default_lang = String::from("text");
        let lang = self.meta.language.as_ref().unwrap_or(&default_lang);

        let pre_attrs = vec![("class", format!("language-{}", lang))];

        let higlighter = Highlighter::default();

        let code = match higlighter.highlight(lang, &self.content) {
            Ok(html) => html,
            Err(e) => e.to_string(),
        };

        fmt.cr();
        fmt.open("pre", &pre_attrs);
        fmt.open("code", &[]);
        fmt.text_raw(&code);
        fmt.close("code");
        fmt.close("pre");
        fmt.cr();
    }
}

struct FancyCodeBlockRule;

impl CoreRule for FancyCodeBlockRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_post_mut(|node, _| {
            let mut meta: Option<Metadata> = None;
            let mut content: Option<&String> = None;

            if let Some(code) = node.cast::<CodeBlock>() {
                content = Some(&code.content);
            }
            if let Some(code) = node.cast::<CodeFence>() {
                meta = Some(Metadata::parse(&code.info));
                content = Some(&code.content);
            }

            if let Some(content) = content {
                node.replace(FancyCodeBlock {
                    meta: meta.unwrap_or_default(),
                    content: String::from(content),
                })
            }
        })
    }
}

pub(super) fn add_code_block_rule(md: &mut MarkdownIt) {
    md.add_rule::<FancyCodeBlockRule>();
}
