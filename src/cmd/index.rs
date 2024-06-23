use std::path::PathBuf;

use crate::{config::SiteConfig, content::Site, error::JellyError, utils::write_file};

pub fn index(source: PathBuf, out: Option<PathBuf>) -> Result<(), JellyError> {
    let config = SiteConfig::new(source);

    let index = Site::build(&config)?.index();
    let json = serde_json::to_string(&index)?;

    if let Some(out) = out {
        write_file(&out, json)?;
    } else {
        println!("{json}");
    }

    Ok(())
}
