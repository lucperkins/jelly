use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;

use crate::config::{SiteConfig, TitleConfig};
use crate::content::{Content, Section};
use crate::error::ContentError;
use crate::page::Page;
use crate::render_page;

#[derive(Serialize)]
pub struct Site {
    pub content: Content,
}

impl Site {
    // TODO: find a less messy, more functional way to aggregate pages
    fn pages(&self) -> Vec<&Page> {
        self.content.pages()
    }
}

pub fn build_site(source: PathBuf, out: PathBuf) -> Result<(), ContentError> {
    let config = SiteConfig {
        root: source,
        title_config: TitleConfig::default(),
    };

    let content = Section::from_path(&config.root, &config)?;

    let site = Site { content };

    for page in site.pages() {
        let html = render_page(page)?;
        let mut path = out.join(&page.relative_path);

        path.set_extension("html");

        let dir = path.as_path().parent().unwrap();
        create_dir_all(dir)?;

        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
    }

    Ok(())
}
