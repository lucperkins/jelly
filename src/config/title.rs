pub(crate) struct TitleConfig {
    pub(crate) title_case: bool,
    pub(crate) first_letter_capitalized: bool,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            title_case: false,
            first_letter_capitalized: true,
        }
    }
}
