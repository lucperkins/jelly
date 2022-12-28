use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    process::ExitCode,
};

use crate::{
    config::{SiteConfig, TitleConfig},
    content::{Section, Site},
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

pub fn build(source: PathBuf, out: PathBuf) -> eyre::Result<ExitCode> {
    let site = build_site(source)?;

    for page in site.pages() {
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
        md::{TableOfContents, TocEntry},
    };

    use super::build_site;

    #[test]
    fn build_real_site() {
        let cases: Vec<(&str, Site)> = vec![(
            "basic",
            Site(Section::new("Welcome", Some(vec![Page {
                path: String::from("tests/full/basic/index.md"),
                relative_path: String::from("index.md"),
                title: String::from("Welcome"),
                body: String::from("# Welcome\n\nWelcome to the site.\n\n## About this site\n\nSome info here."),
                html: String::from("<h1>Welcome</h1>\n<p>Welcome to the site.</p>\n<h2>About this site</h2>\n<p>Some info here.</p>\n"),
                breadcrumb: vec![Link {
                    path: PathBuf::from("tests/full/basic"),
                    title: String::from("Welcome"),
                }],
                table_of_contents: TableOfContents(vec![TocEntry::new(2, "About this site", TableOfContents::empty())]),
            }]), None)),
        ),
        ("medium", Site(Section::new("Welcome", Some(vec![Page {
            path: String::from("tests/full/medium/index.md"),
            relative_path: String::from("index.md"),
            title: String::from("Welcome"),
            body: String::from("# Welcome\n\nWelcome to the site.\n\n## About this site\n\nSome info here."),
            html: String::from("<h1>Welcome</h1>\n<p>Welcome to the site.</p>\n<h2>About this site</h2>\n<p>Some info here.</p>\n"),
            breadcrumb: vec![Link {
                path: PathBuf::from("tests/full/medium"),
                title: String::from("Welcome"),
            }],
            table_of_contents: TableOfContents(vec![TocEntry::new(2, "About this site", TableOfContents::empty())]),
        }]), None)))];

        for (dir, expected_site) in cases {
            let project_dir = format!("tests/full/{}", dir);
            let site = build_site(PathBuf::from(project_dir)).unwrap();
            assert_eq!(site, expected_site);
        }
    }
}
