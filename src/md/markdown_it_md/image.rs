use markdown_it::{
    generics::inline::full_link, parser::inline::Text, MarkdownIt, Node, NodeValue, Renderer,
};

#[derive(Debug)]
struct FancyImage {
    url: String,
    title: Option<String>,
}

impl NodeValue for FancyImage {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut img_attrs = vec![("src", self.url.to_string())];
        if let Some(title) = &self.title {
            img_attrs.push(("title", title.to_string()));
        }
        let a_attrs = &[("href", self.url.to_string())];

        let mut alt = String::new();
        node.walk(|node, _| {
            if let Some(text) = node.cast::<Text>() {
                alt.push_str(text.content.as_str());
            }
        });
        if !alt.is_empty() {
            img_attrs.push(("alt", alt.to_string()));
        }

        fmt.open("figure", &[]);
        fmt.open("a", a_attrs);
        fmt.self_close("img", &img_attrs);
        fmt.close("a");
        fmt.close("figure");
    }
}

pub(super) fn add_image_rule(md: &mut MarkdownIt) {
    full_link::add_prefix::<'!', true>(md, |href, title| {
        Node::new(FancyImage {
            url: href.unwrap_or("".to_string()),
            title,
        })
    });
}

#[cfg(test)]
mod tests {
    use crate::tests::test_markdown_produces_expected_html;

    #[test]
    fn image_render() {
        let cases: Vec<(&str, &str)> = vec![(
            "![](https://example.com/foo.png)",
            "<p><figure><a href=\"https://example.com/foo.png\"><img src=\"https://example.com/foo.png\"></a></figure></p>\n",
        ), (
            "![Some title](https://example.com/foo.png \"bar\")",
            "<p><figure><a href=\"https://example.com/foo.png\"><img src=\"https://example.com/foo.png\" title=\"bar\" alt=\"Some title\"></a></figure></p>\n",
        )];

        test_markdown_produces_expected_html(cases);
    }
}
