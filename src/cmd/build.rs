use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use crate::{
    config::{SiteConfig, TitleConfig},
    content::Section,
    error::Error,
    render_page,
    search::build_search_index_for_page,
    site::Site,
};

pub fn build_site(source: PathBuf, out: PathBuf) -> Result<(), Error> {
    let config = SiteConfig {
        root: source,
        title_config: TitleConfig::default(),
    };

    let content = Section::from_path(&config.root, None, &config)?;

    let site = Site { content };

    for page in site.pages() {
        let html = render_page(page)?;
        let mut path = out.join(&page.relative_path);

        let _ = build_search_index_for_page(page);

        path.set_extension("html");

        let dir = path.as_path().parent().unwrap();
        create_dir_all(dir)?;

        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
    }

    Ok(())
}
