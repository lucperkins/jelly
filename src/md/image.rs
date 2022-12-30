use markdown_it::{
    parser::{core::CoreRule, inline::Text},
    plugins::cmark::inline::image::Image,
    MarkdownIt, Node, NodeValue, Renderer,
};

#[derive(Debug)]
struct FancyImage {
    url: String,
    title: Option<String>,
}

impl NodeValue for FancyImage {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut img_attrs = vec![("src", self.url.clone())];
        if let Some(title) = &self.title {
            img_attrs.push(("title", title.clone()));
        }
        let a_attrs = vec![("href", self.url.clone())];

        let mut alt = String::new();
        node.walk(|node, _| {
            if let Some(text) = node.cast::<Text>() {
                alt.push_str(text.content.as_str());
            }
        });
        img_attrs.push(("alt", alt));

        fmt.cr();
        fmt.open("figure", &[]);
        fmt.open("a", &a_attrs);
        fmt.self_close("img", &img_attrs);
        fmt.close("a");
        fmt.close("figure");
        fmt.cr();
    }
}

struct FancyImageScanner;

impl CoreRule for FancyImageScanner {
    fn run(node: &mut Node, _: &MarkdownIt) {
        node.walk_mut(|node, _| {
            if let Some(image) = node.cast::<Image>() {
                let img_node = FancyImage {
                    title: image.title.clone(),
                    url: image.url.clone(),
                };

                node.replace(img_node);
            }
        });
    }
}

pub fn add_image_rule(md: &mut MarkdownIt) {
    md.add_rule::<FancyImageScanner>();
}

#[cfg(test)]
mod tests {
    use crate::md::{ast, render};

    #[test]
    fn image_render() {
        let cases: Vec<(&str, &str)> = vec![(
            r#"![Some title](https://example.com/foo.png "Something else")"#,
            r#"<figure><a href="https://example.com/foo.png"><img src="https://example.com/foo.png" /></a></figure>"#,
        )];

        for (md, expected_html) in cases {
            let html = render(&&ast(md));
            assert_eq!(html, expected_html);
        }
    }
}
