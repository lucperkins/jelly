use crate::page::TitleConfig;
use std::path::Path;
use titlecase::titlecase;

pub fn name_from_path(
    maybe_title: Option<String>,
    path: &Path,
    title_config: &TitleConfig,
) -> String {
    maybe_title.unwrap_or_else(|| {
        let stem = path.file_stem().unwrap();
        let stem_str = stem.to_string_lossy();

        #[allow(clippy::single_char_pattern)]
        let deslugged = stem_str.replace("-", " ");

        if title_config.title_case {
            titlecase(&deslugged)
        } else if title_config.first_letter_capitalized {
            capitalize_first_letter(&deslugged)
        } else {
            deslugged
        }
    })
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}
