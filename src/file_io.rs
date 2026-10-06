use std::fs;
const ROOT: &str = ".formats/";
const LF: &str = "language.json"; // language file
const REGION: &str = "region/";
const CURRENCY: &str = "currency.json";
const DATE_FORMAT: &str = "date.json";

pub fn load_language(lang: &str) -> String {
    let format = format!("{}{}/{}", ROOT, lang, LF);
    let loaded = fs::read_to_string(&format).expect("failed to load language");
    loaded
}

pub fn load_currency(reg: &str) -> String {
    let lang = &reg[..2];
    let format = format!("{}{}/{}{}/{}", ROOT, lang, REGION, reg, CURRENCY);
    let loaded = fs::read_to_string(&format).expect("failed to load currency");
    loaded
}

pub fn load_date_format(reg: &str) -> String {
    let lang = &reg[..2];
    let format = format!("{}{}/{}{}/{}", ROOT, lang, REGION, reg, DATE_FORMAT);
    let loaded = fs::read_to_string(&format).expect("failed to load date format");
    loaded
}
