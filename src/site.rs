use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::config::{SiteConfig, TitleConfig};
use crate::content::{get_section, Content, Section};
use crate::error::ContentError;
use crate::page::Page;
use crate::render_page;

#[derive(Serialize)]
pub struct Site {
    pub content: Content,
}

fn handle_section<'a>(s: &'a Section) -> Vec<&'a Page> {
    let mut pages: Vec<&'a Page> = Vec::new();

    if let Some(ps) = &s.pages {
        for p in ps {
            pages.push(&p);
        }
    }

    if let Some(ss) = &s.sections {
        for s in ss {
            let ps = handle_section(&s);
            for p in ps {
                pages.push(p);
            }
        }
    }

    pages
}

impl Site {
    fn pages(&self) -> Vec<&Page> {
        let mut pages: Vec<&Page> = Vec::new();

        if let Some(ps) = &self.content.pages {
            for page in ps {
                pages.push(page);
            }
        }

        if let Some(sections) = &self.content.sections {
            for section in sections {
                let ps = handle_section(section);
                for p in ps {
                    pages.push(p);
                }
            }
        }

        pages
    }
}

pub fn build_site(source: PathBuf) -> Result<(), ContentError> {
    let config = SiteConfig {
        root: source,
        title_config: TitleConfig::default(),
    };

    let content = get_section(&config.root, &config)?;

    let site = Site { content };

    for page in site.pages() {
        let html = render_page(&page)?;
        let mut path = Path::new("dist").join(&page.relative_path);

        path.set_extension("html");

        let dir = path.as_path().parent().unwrap();
        create_dir_all(dir)?;

        let mut file = File::create(path)?;
        file.write_all(&html.as_bytes())?;
    }

    Ok(())
}
