use std::path::PathBuf;

use crate::{config::SiteConfig, content::Site, error::JellyError};

pub fn build(source: PathBuf, out: PathBuf, sanitize: bool) -> Result<(), JellyError> {
    Site::write(&SiteConfig::new(source), out, sanitize)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        config::SiteConfig,
        content::{Link, Page, Section, Site},
        md::{SearchDocument, SearchIndex, TableOfContents, TocEntry},
    };

    #[test]
    fn build_real_site() {
        let cases: Vec<(&str, Site)> = vec![
            (
                "basic",
                Site(Section::new(
                    "Welcome",
                    "/",
                    Some(vec![
                        Page::new(
                            "tests/full/basic/contact.md",
                            "contact.md",
                            "contact",
                            "Contact us",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![Link::new(&PathBuf::from("tests/full/basic"), "Welcome")],
                            TableOfContents::new(vec![]),
                            SearchIndex(vec![SearchDocument::new(
                                1,
                                "Contact us",
                                "Contact us",
                                "Send us a fax.",
                            )]),
                            Some(1),
                        ),
                        Page::new(
                            "tests/full/basic/index.md",
                            "index.md",
                            "",
                            "Welcome",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![Link::new(&PathBuf::from("tests/full/basic"), "Welcome")],
                            TableOfContents::new(vec![TocEntry::new(
                                2,
                                "About this site",
                                TableOfContents::empty(),
                            )]),
                            SearchIndex(vec![
                                SearchDocument::new(
                                    1,
                                    "Welcome",
                                    "Welcome",
                                    "Welcome to the site.",
                                ),
                                SearchDocument::new(
                                    2,
                                    "Welcome",
                                    "About this site",
                                    "Some info here.",
                                ),
                            ]),
                            Some(5),
                        ),
                        Page::new(
                            "tests/full/basic/about.md",
                            "about.md",
                            "about",
                            "About",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![Link::new(&PathBuf::from("tests/full/basic"), "Welcome")],
                            TableOfContents::new(vec![]),
                            SearchIndex(vec![SearchDocument::new(
                                1,
                                "About",
                                "About",
                                "About this thing.",
                            )]),
                            Some(2),
                        ),
                    ]),
                    None,
                )),
            ),
            (
                "medium",
                Site(Section::new(
                    "Medium-sized project",
                    "/",
                    Some(vec![Page::new(
                        "tests/full/medium/index.md",
                        "index.md",
                        "",
                        "Welcome",
                        "", // Omit for testing
                        "", // Omit for testing
                        vec![Link::new(
                            &PathBuf::from("tests/full/medium"),
                            "Medium-sized project",
                        )],
                        TableOfContents::new(vec![TocEntry::new(
                            2,
                            "About this site",
                            TableOfContents::empty(),
                        )]),
                        SearchIndex(vec![
                            SearchDocument::new(1, "Welcome", "Welcome", "Welcome to the site."),
                            SearchDocument::new(2, "Welcome", "About this site", "Some info here."),
                        ]),
                        None,
                    )]),
                    Some(vec![Section::new(
                        "Setup",
                        "/setup",
                        Some(vec![Page::new(
                            "tests/full/medium/setup/index.md",
                            "setup/index.md",
                            "setup",
                            "Setup",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![
                                Link::new(
                                    &PathBuf::from("tests/full/medium"),
                                    "Medium-sized project",
                                ),
                                Link::new(&PathBuf::from("tests/full/medium/setup"), "Setup"),
                            ],
                            TableOfContents::empty(),
                            SearchIndex(vec![SearchDocument::new(
                                1,
                                "Setup",
                                "Setup",
                                "Here is how to set things up. Here is some other info.",
                            )]),
                            None,
                        )]),
                        None,
                    )]),
                )),
            ),
        ];

        for (dir, expected_site) in cases {
            let project_dir = format!("tests/full/{}", dir);
            let config = SiteConfig::new(PathBuf::from(project_dir));

            let content = Site::build(&config).unwrap().0;

            for (idx, page) in content.pages().iter().enumerate() {
                let expected = expected_site.pages()[idx];

                assert_eq!(page.path, expected.path);
                assert_eq!(page.relative_path, expected.relative_path);
                assert_eq!(page.title, expected.title);
                assert_eq!(page.breadcrumb, expected.breadcrumb);
                assert_eq!(page.table_of_contents, expected.table_of_contents);
                assert_eq!(page.search_index, expected.search_index);
                assert_eq!(page.order, expected.order);
            }
        }
    }
}
