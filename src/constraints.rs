use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static VOWELS: &str = "aeiou";
pub static CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

pub static CASE_SENSITIVE: &str = "oiudgjnrstyz";
pub static SORBORNO: &str = "অআইঈউঊঊএঐওঔ";
pub static KAR: Lazy<Box<[&str]>> =
    Lazy::new(|| Box::new(["া", "ি", "ী", "ু", "ূ", "ৃ", "ে", "ৈ", "ো", "ৌ"]));
pub static IGNORED_PUNCTUATION_MARKS: Lazy<Box<[&str]>> =
    Lazy::new(|| Box::new(["ঁ", "।", "?", ".", "-", ";"]));

pub static REMAPPED_WORDS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    return HashMap::from([("ফেসবুক", "Facebook"), ("গুগল", "Google")]);
});

pub static NUMBERS: &str = "0123456789";
