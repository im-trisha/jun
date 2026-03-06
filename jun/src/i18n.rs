use serde::{Deserialize, Serialize};
use typed_i18n::TypedI18N;

#[derive(Copy, Clone, Serialize, Deserialize, TypedI18N)]
#[typed_i18n(filename = "assets/i18n.yaml")]
#[typed_i18n(builder = "mixed_str", prefix = "t_")]
#[serde(rename_all = "snake_case")]
pub enum Language {
    It,
    #[serde(other)] // defaults to english if an invalid variant is found
    En,
}

impl Language {
    pub fn from_locale(locale: &str) -> Self {
        match locale.split('-').next().unwrap_or("") {
            "it" => Language::It,
            "en" => Language::En,
            _ => Language::En,
        }
    }
}
