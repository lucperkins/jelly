use std::{
    cmp::Ordering,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    process::ExitCode,
};

use crate::{
    config::{SiteConfig, TitleConfig},
    content::{Page, Section, Site},
    error::Error,
    md::render_page,
};

fn build_site(source: PathBuf) -> Result<Site, Error> {
    let config = SiteConfig {
        root: source,
        title_config: TitleConfig::default(),
    };

    let content = Section::from_path(&config.root, None, &config)?;

    Ok(Site(content))
}

fn pages_by_title(a: &&Page, b: &&Page) -> Ordering {
    if a.title < b.title {
        Ordering::Greater
    } else if a.title == b.title {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

pub fn build(source: PathBuf, out: PathBuf) -> eyre::Result<ExitCode> {
    let site = build_site(source)?;

    let mut pages = site.pages();
    pages.sort();
    pages.sort_by(pages_by_title);

    for page in pages {
        let html = render_page(page)?;
        let mut path = out.join(&page.relative_path);

        path.set_extension("html");

        let dir = path.as_path().parent().unwrap();
        create_dir_all(dir)?;

        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
    }

    Ok(ExitCode::SUCCESS)
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
                            "Contact us",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![Link::new(&PathBuf::from("tests/full/basic"), "Welcome")],
                            TableOfContents(vec![]),
                            SearchIndex(vec![]),
                            Some(1),
                        ),
                        Page::new(
                            "tests/full/basic/index.md",
                            "index.md",
                            "Welcome",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![Link::new(&PathBuf::from("tests/full/basic"), "Welcome")],
                            TableOfContents(vec![TocEntry::new(
                                2,
                                "About this site",
                                TableOfContents::empty(),
                            )]),
                            SearchIndex(vec![SearchDocument::new(
                                2,
                                "Welcome",
                                "About this site",
                                "Some info here.",
                            )]),
                            None,
                        ),
                        Page::new(
                            "tests/full/basic/about.md",
                            "about.md",
                            "About",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![Link::new(&PathBuf::from("tests/full/basic"), "Welcome")],
                            TableOfContents(vec![]),
                            SearchIndex(vec![]),
                            Some(2),
                        ),
                    ]),
                    None,
                )),
            ),
            (
                "medium",
                Site(Section::new(
                    "Documentation",
                    Some(vec![Page::new(
                        "tests/full/medium/index.md",
                        "index.md",
                        "Welcome",
                        "", // Omit for testing
                        "", // Omit for testing
                        vec![Link::new(
                            &PathBuf::from("tests/full/medium"),
                            "Documentation",
                        )],
                        TableOfContents(vec![TocEntry::new(
                            2,
                            "About this site",
                            TableOfContents::empty(),
                        )]),
                        SearchIndex(vec![SearchDocument::new(
                            2,
                            "Welcome",
                            "About this site",
                            "Some info here.",
                        )]),
                        None,
                    )]),
                    Some(vec![Section::new(
                        "Setup",
                        Some(vec![Page::new(
                            "tests/full/medium/setup/index.md",
                            "setup/index.md",
                            "Setup",
                            "", // Omit for testing
                            "", // Omit for testing
                            vec![
                                Link::new(&PathBuf::from("tests/full/medium"), "Documentation"),
                                Link::new(&PathBuf::from("tests/full/medium/setup"), "Setup"),
                            ],
                            TableOfContents::empty(),
                            SearchIndex::empty(),
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
