use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use serde_json::Value;

#[derive(RustEmbed)]
#[folder = "resources/"]
struct ResourceDirectory;

pub static VOWELS: &str = "aeiou";
pub static CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

pub static CASE_SENSITIVE: &str = "oiudgjnrstyz";
pub static SORBORNO: &str = "অআইঈউঊঊএঐওঔ";
pub static KAR: &'static [&str] = &["া", "ি", "ী", "ু", "ূ", "ৃ", "ে", "ৈ", "ো", "ৌ"];
pub static IGNORED_PUNCTUATION_MARKS: &'static [&str] = &["ঁ", "।", "?", ".", "-", ";"];
pub static NUMBERS: &str = "0123456789";

pub static REMAPPED_WORDS: Lazy<Value> = Lazy::new(|| {
    return serde_json::from_str(
        std::str::from_utf8(&ResourceDirectory::get("remapped.json").unwrap().data).unwrap(),
    )
    .unwrap();
});
pub static JSON_DATA: Lazy<serde_json::Value> = Lazy::new(|| {
    return serde_json::from_str(
        std::str::from_utf8(&ResourceDirectory::get("data.json").unwrap().data).unwrap(),
    )
    .unwrap();
});
