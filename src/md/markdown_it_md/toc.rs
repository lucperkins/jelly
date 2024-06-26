// This approach is deeply indebted to how rustdoc does it. See the code here:
// https://github.com/rust-lang/rust/blob/master/src/librustdoc/html/toc.rs
// In particular, the `fold_until` approach offered me a way out of a thorny nested
// recursion problem.

use markdown_it::Node;
use serde::Serialize;
use slug::slugify;

use super::headings::Headings;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct TableOfContents {
    pub(super) entries: Vec<TocEntry>,
}

impl TableOfContents {
    pub(crate) fn parse(document: &Node) -> Self {
        let mut builder = TocBuilder::new();

        for heading in Headings(&document.children) {
            builder.push(heading.level, heading.text);
        }

        builder.into_toc()
    }

    pub(crate) fn empty() -> Self {
        Self { entries: vec![] }
    }

    #[cfg(test)]
    pub(crate) fn new(entries: Vec<TocEntry>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct TocEntry {
    level: u8,
    text: String,
    slug: String,
    children: TableOfContents,
}

impl TocEntry {
    pub(crate) fn new(level: u8, text: &str, children: TableOfContents) -> Self {
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
            top_level: TableOfContents {
                entries: Vec::new(),
            },
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
                    next.children.entries.extend(this);
                    if next.level < level {
                        self.chain.push(next);
                        return;
                    } else {
                        this = Some(next);
                    }
                }
                None => {
                    self.top_level.entries.extend(this);
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
            TableOfContents::new(vec![
                TocEntry::new(
                    2,
                    "Now a heading 2",
                    TableOfContents::new(vec![TocEntry::new(
                        3,
                        "Now a heading 3",
                        TableOfContents::new(vec![TocEntry::new(
                            4,
                            "Let's go even deeper",
                            TableOfContents::empty(),
                        )]),
                    )]),
                ),
                TocEntry::new(
                    2,
                    "And now back to a heading 2",
                    TableOfContents::new(vec![TocEntry::new(
                        3,
                        "Another heading 3",
                        TableOfContents::empty(),
                    )]),
                ),
                TocEntry::new(
                    2,
                    "And yet another heading 2",
                    TableOfContents::new(vec![TocEntry::new(
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
