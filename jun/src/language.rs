use typed_i18n::TypedI18N;

#[derive(Copy, Clone, TypedI18N)]
#[typed_i18n(filename = "i18n.yaml")]
#[typed_i18n(builder = "mixed_str", prefix = "t_")]
pub enum Language {
    En,
    It,
}
