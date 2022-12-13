use crate::constraints;

fn replace(string: &str) -> &str {
    let strings_to_vector: Vec<char> = string.chars().collect();

    let mut new_bangla_vector_string: Vec<&str> = Vec::new();

    for character in strings_to_vector {
        let char = character.to_string();
        let mut replaced_word = constraints::REPLACE_PATTERNS.get(&*char);
        // The normal 'H' is not in the replace_patterns
        // Lets try with the lower case version
        if replaced_word == None {
            if character.is_uppercase() {
                replaced_word = constraints::REPLACE_PATTERNS.get(&*char.to_lowercase());
            } else {
                replaced_word = constraints::REPLACE_PATTERNS.get(&*char.to_uppercase());
            }
        }
        let replaced_value = *replaced_word.unwrap();
        new_bangla_vector_string.push(replaced_value)
    }

    let new_bangla_word = new_bangla_vector_string.join("");
    print!("{}", new_bangla_word.as_str());

    return string;
}

pub fn parse(parsable_string: &str) -> &str {
    // First split the string by whitespace
    // And check if theres a possible way to replace words

    let sub_strings = parsable_string.split_whitespace(); // ['Hello','world'],
    for byte in sub_strings {
        // We get "Hello" here
        replace(byte);
        break;
    }

    return parsable_string;
}
