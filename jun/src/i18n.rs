use serde::{Deserialize, Serialize};
use typed_i18n::TypedI18N;

#[derive(Copy, Clone, Serialize, Deserialize, TypedI18N)]
#[typed_i18n(filename = "assets/i18n.yaml", separator = "_")]
#[typed_i18n(builder = "mixed_str", prefix = "t_")]
#[serde(rename_all = "snake_case")]
pub enum Language {
    It,
    #[serde(other)] // defaults to english if an invalid variant is found
    En,
}

impl Language {
    pub fn from_locale(locale: &str) -> Option<Self> {
        match locale.split('-').next().unwrap_or("") {
            "it" => Some(Language::It),
            "en" => Some(Language::En),
            _ => None,
        }
    }
}
