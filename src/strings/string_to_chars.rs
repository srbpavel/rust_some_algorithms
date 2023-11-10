//
pub fn string_to_chars(s: &str) -> Vec<char> {
    s
        .chars()
        .collect::<Vec<char>>()
}

//
pub fn string_to_chars_uppercase(s: &str) -> Vec<char> {
    s
        // we can also convert str
        //.to_uppercase()
        .chars()
        // on each char
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>()
}

//
pub fn who_am_i() -> Option<std::path::PathBuf> {
    let this_file = std::file!();

    let filename = std::path::Path::new(this_file);

    std::fs::canonicalize(filename)
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fish() {
        let input = "fish";
        let output = vec!['f', 'i', 's', 'h'];

        let result = string_to_chars(input);
        assert_eq!(result, output);
    }
}
