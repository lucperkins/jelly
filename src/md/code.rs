use markdown_it::{
    parser::core::CoreRule,
    plugins::cmark::block::{code::CodeBlock, fence::CodeFence},
    MarkdownIt, Node, NodeValue, Renderer,
};

use super::highlight::Highlighter;

#[derive(Debug)]
pub struct FancyCodeBlock {
    meta: Option<String>,
    pub content: String,
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
        fmt.text_raw(&code);
        fmt.close("code");
        fmt.close("pre");
        fmt.cr();
    }
}

pub struct FancyCodeBlockRule;

impl CoreRule for FancyCodeBlockRule {
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

pub fn add_code_block_rule(md: &mut MarkdownIt) {
    md.add_rule::<FancyCodeBlockRule>();
}
