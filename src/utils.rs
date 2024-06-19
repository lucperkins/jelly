use crate::config::TitleConfig;
use crate::error::JellyError;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use titlecase::titlecase;

pub(super) fn name_from_path(path: &Path, title_config: &TitleConfig) -> String {
    let stem_str = path.to_string_lossy();

    let deslugged = stem_str.replace('-', " ");

    if title_config.title_case {
        titlecase(&deslugged)
    } else if title_config.first_letter_capitalized {
        capitalize_first_letter(&deslugged)
    } else {
        deslugged
    }
}

pub(super) fn get_file(path: &Path) -> Result<String, JellyError> {
    let mut file = File::open(path.as_os_str())?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents)?;
    Ok(contents)
}

// TODO: find a built-in function for this
pub(super) fn get_or_none<T>(items: Vec<T>) -> Option<Vec<T>> {
    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}
