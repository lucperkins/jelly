use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use ammonia::clean;

use crate::{
    config::{SiteConfig, TitleConfig},
    content::{Section, Site},
    error::JellyError,
    md::render_page,
    utils::write_file,
};

use super::index::SiteIndex;

pub(crate) fn build_site(source: PathBuf) -> Result<Site, JellyError> {
    let config = SiteConfig {
        root: source,
        title_config: TitleConfig::default(),
    };

    let content = Section::from_path(&config.root, None, &config)?;

    Ok(Site(content))
}

pub fn build(source: &PathBuf, out: &Path, sanitize: bool) -> Result<(), JellyError> {
    let site = build_site(source.into())?;

    let attrs = site.attrs()?;

    for page in site.pages() {
        let html = render_page(page, attrs.clone())?;
        let mut path = out.join(&page.relative_path);

        path.set_extension("html");

        if let Some(dir) = path.as_path().parent() {
            create_dir_all(dir)?;
        }

        let final_html = if sanitize { clean(&html) } else { html };

        write_file(&path, final_html)?;
    }

    let search_index = SiteIndex::new(site.documents());
    let search_index_json = serde_json::to_string(&search_index)?;
    let index_file_path = out.join("search.json");

    write_file(&index_file_path, search_index_json)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        content::{Link, Page, Section, Site},
        md::{SearchDocument, SearchIndex, TableOfContents, TocEntry},
    };

    use super::build_site;

    #[test]
    fn build_real_site() {
        let cases: Vec<(&str, Site)> = vec![
            (
                "basic",
                Site(Section::new(
                    "Welcome",
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
            let content = build_site(PathBuf::from(project_dir)).unwrap().0;

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
