// This approach is deeply indebted to how rustdoc does it. See the code here:
// https://doc.rust-lang.org/stable/nightly-rustc/src/rustdoc/html/toc.rs.html#1-191
// In particular, the `fold_until` approach offered me a way out of a thorny nested
// recursion problem.

use markdown_it::Node;
use serde::Serialize;
use slug::slugify;

use super::headings::Headings;

#[derive(Debug, PartialEq, Serialize)]
pub struct TableOfContents(Vec<TocEntry>);

impl TableOfContents {
    pub fn parse(document: &Node) -> Self {
        let mut builder = TocBuilder::new();

        for heading in Headings(&document.children) {
            builder.push(heading.level, heading.text);
        }

        builder.into_toc()
    }

    fn empty() -> Self {
        Self(vec![])
    }
}

#[derive(Debug, PartialEq, Serialize)]
struct TocEntry {
    level: u8,
    text: String,
    slug: String,
    children: TableOfContents,
}

impl TocEntry {
    fn new(level: u8, text: &str, children: TableOfContents) -> Self {
        let slug = slugify(text);
        Self {
            level,
            text: String::from(text),
            slug,
            children,
        }
    }
}

#[derive(PartialEq)]
struct TocBuilder {
    top_level: TableOfContents,
    chain: Vec<TocEntry>,
}

impl TocBuilder {
    fn new() -> Self {
        Self {
            top_level: TableOfContents(Vec::new()),
            chain: Vec::new(),
        }
    }

    fn into_toc(mut self) -> TableOfContents {
        self.fold_until(1); // Don't include h1
        self.top_level
    }

    fn fold_until(&mut self, level: u8) {
        let mut this = None;
        loop {
            match self.chain.pop() {
                Some(mut next) => {
                    next.children.0.extend(this);
                    if next.level < level {
                        self.chain.push(next);
                        return;
                    } else {
                        this = Some(next);
                    }
                }
                None => {
                    self.top_level.0.extend(this);
                    return;
                }
            }
        }
    }

    fn push(&mut self, level: u8, text: String) {
        assert!(level >= 2);

        self.fold_until(level);

        self.chain
            .push(TocEntry::new(level, &text, TableOfContents::empty()));
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::md::ast;

    use super::{TableOfContents, TocEntry};

    #[test]
    fn create_toc() {
        let cases: Vec<(&str, TableOfContents)> = vec![(
            indoc! {"
                # This gets ignored

                Here is some text.

                ## Now a heading 2

                More text.

                ### Now a heading 3

                More text.

                #### Let's go even deeper

                Filler here.

                ## And now back to a heading 2

                More text.

                ### Another heading 3

                ## And yet another heading 2

                #### Let's skip a level
            "},
            TableOfContents(vec![
                TocEntry::new(
                    2,
                    "Now a heading 2",
                    TableOfContents(vec![TocEntry::new(
                        3,
                        "Now a heading 3",
                        TableOfContents(vec![TocEntry::new(
                            4,
                            "Let's go even deeper",
                            TableOfContents::empty(),
                        )]),
                    )]),
                ),
                TocEntry::new(
                    2,
                    "And now back to a heading 2",
                    TableOfContents(vec![TocEntry::new(
                        3,
                        "Another heading 3",
                        TableOfContents::empty(),
                    )]),
                ),
                TocEntry::new(
                    2,
                    "And yet another heading 2",
                    TableOfContents(vec![TocEntry::new(
                        4,
                        "Let's skip a level",
                        TableOfContents::empty(),
                    )]),
                ),
            ]),
        )];

        for (md, expected_toc) in cases {
            let tree = ast(md);
            let toc = TableOfContents::parse(&tree);
            assert_eq!(toc, expected_toc);
        }
    }
}
