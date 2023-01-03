use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::{
    config::{SectionConfig, SiteConfig, TitleConfig},
    error::Error,
    md::get_document_title,
    utils::{get_file, name_from_path},
};

use super::front::FrontMatter;

pub fn infer_page_title(
    front: FrontMatter,
    path: &Path,
    file: String,
    title_config: &TitleConfig,
) -> String {
    front.title.unwrap_or_else(|| {
        get_document_title(&file).unwrap_or_else(|| name_from_path(path, title_config))
    })
}

fn title_from_index_page(path: &Path) -> Result<Option<String>, Error> {
    let index_path = Path::new(&path).join("index.md");
    if index_path.exists() {
        let file = get_file(&index_path)?;
        match get_document_title(&file) {
            Some(t) => Ok(Some(t)),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn get_section_title(path: &PathBuf, config: &SiteConfig) -> Result<String, Error> {
    let yaml_file_path = Path::new(&path).join("_dir.yaml");
    if yaml_file_path.exists() {
        let yaml_file_str = read_to_string(&yaml_file_path)?;
        let section_config: SectionConfig = serde_yaml::from_str(&yaml_file_str)?;
        match section_config.title {
            Some(t) => Ok(t),
            None => {
                let t = title_from_index_page(path)?;
                let title = t.unwrap_or_else(|| name_from_path(path, &config.title_config));
                Ok(title)
            }
        }
    } else {
        let t = title_from_index_page(path)?;
        let title = t.unwrap_or_else(|| name_from_path(path, &config.title_config));
        Ok(title)
    }
}
