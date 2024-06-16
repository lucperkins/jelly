use serde::Serialize;
use std::{fs::File, io::Write, path::PathBuf};

use crate::{error::Error, md::SearchDocument};

use super::build::build_site;

#[derive(Serialize)]
pub struct SiteIndex {
    documents: Vec<SearchDocument>,
}

impl SiteIndex {
    pub fn new(documents: Vec<SearchDocument>) -> Self {
        Self { documents }
    }
}

pub fn index(source: PathBuf, out: Option<PathBuf>) -> Result<(), Error> {
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
