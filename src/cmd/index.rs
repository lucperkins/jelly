use serde::Serialize;
use std::{fs::File, io::Write, path::PathBuf, process::ExitCode};

use crate::md::SearchDocument;

use super::build::build_site;

#[derive(Serialize)]
struct SiteIndex {
    documents: Vec<SearchDocument>,
}

pub fn index(source: PathBuf, out: Option<PathBuf>) -> eyre::Result<ExitCode> {
    let site = build_site(source)?;
    let documents = site.documents();
    let site_index = SiteIndex { documents };
    let json = serde_json::to_string(&site_index)?;

    if let Some(out) = out {
        let mut file = File::create(out)?;
        file.write_all(json.as_bytes())?;
    } else {
        println!("{json}");
    }

    Ok(ExitCode::SUCCESS)
}
