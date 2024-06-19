use serde::Serialize;
use std::{fs::File, io::Write, path::PathBuf};

use crate::{error::JellyError, md::SearchDocument};

use super::build::build_site;

pub fn index(source: PathBuf, out: Option<PathBuf>) -> Result<(), JellyError> {
    let site = build_site(source)?;
    let site_index = SiteIndex::new(site.documents());
    let json = serde_json::to_string(&site_index)?;

    if let Some(out) = out {
        let mut file = File::create(out)?;
        file.write_all(json.as_bytes())?;
    } else {
        println!("{json}");
    }

    Ok(())
}

#[derive(Serialize)]
pub(super) struct SiteIndex {
    documents: Vec<SearchDocument>,
}

impl SiteIndex {
    pub(super) fn new(documents: Vec<SearchDocument>) -> Self {
        Self { documents }
    }
}
