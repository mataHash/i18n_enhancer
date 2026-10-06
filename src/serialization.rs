use crate::types::{Currency, DateFormat, Language};
pub fn ser_lang(entity: &str) -> Language {
    let en: Language = serde_json::from_str(entity).expect("failed to deserialize");
    en
}

pub fn ser_curr(entity: &str) -> Currency {
    let en: Currency = serde_json::from_str(entity).expect("failed to deserialize");
    en
}

pub fn ser_format(entity: &str) -> DateFormat {
    let en: DateFormat = serde_json::from_str(entity).expect("failed to deserialize");
    en
}
