pub fn parse(parsable_string: &str) -> &str {
    // First split the string by whitespace
    // And check if theres a possible way to replace words
    let sub_strings = parsable_string.split_whitespace();
    for byte in sub_strings {
        println!("{}", byte);
    }

    return parsable_string;
}
