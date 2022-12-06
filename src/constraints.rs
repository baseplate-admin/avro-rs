use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static VOWELS: &str = "aeiou";
pub static CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

pub static CASE_SENSITIVE: &str = "oiudgjnrstyz";
pub static SORBORNO: &str = "অআইঈউঊঊএঐওঔ";
pub static KAR: &'static [&str] = &["া", "ি", "ী", "ু", "ূ", "ৃ", "ে", "ৈ", "ো", "ৌ"];
pub static IGNORED_PUNCTUATION_MARKS: &'static [&str] = &["ঁ", "।", "?", ".", "-", ";"];

pub static REMAPPED_WORDS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    return HashMap::from([
        // Remap everything here
        ("ফেসবুক", "Facebook"),
        ("গুগল", "Google"),
        ("উইকিপিডিয়া", "Wikipedia"),
    ]);
});

pub static NUMBERS: &str = "0123456789";

pub static REPLACE_PATTERNS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    return HashMap::from([
        // ("bhl", "ভ্ল"),
        ("psh", "পশ"),
    ]);
});
