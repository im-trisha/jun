use serde::{Deserialize, Serialize};
use typed_i18n::TypedI18N;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, TypedI18N)]
#[typed_i18n(filename = "assets/i18n.yaml", separator = "_")]
#[typed_i18n(builder = "mixed_str", prefix = "t_")]
#[serde(rename_all = "snake_case")]
pub enum Language {
    It,
    #[serde(other)] // defaults to english if an invalid variant is found
    En,
}

impl Language {
    pub const VALUES: &'static [Self] = &[Self::It, Self::En];

    #[must_use] 
    pub fn from_locale(locale: &str) -> Option<Self> {
        match locale.split('-').next().unwrap_or("") {
            "it" => Some(Self::It),
            "en" => Some(Self::En),
            _ => None,
        }
    }
    #[must_use] 
    pub fn name(&self) -> &'static str {
        match self {
            Self::It => self.t_app_languages_it(),
            Self::En => self.t_app_languages_en(),
        }
    }
}
