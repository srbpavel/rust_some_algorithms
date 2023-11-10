//
pub fn string_to_chars(s: &str) -> Vec<char> {
    s
        .chars()
        .collect::<Vec<char>>()
}

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
//pub fn who_am_i() -> String {
//pub fn who_am_i() -> Option<&'static str> {
//pub fn who_am_i() -> Result<std::path::PathBuf, std::io::Error> {
pub fn who_am_i() -> Option<std::path::PathBuf> {
    let this_file = std::file!();

    let filename = std::path::Path::new(this_file);
        /* // just parent with no filename
        .parent()
        .and_then(|s| s.to_str());
        */
        
        /* // src/..
        .to_str();
        */
        
        /* // only filename
        // Option<&OsStr>
        .file_name()
        // Option<&str>
        .and_then(|s| s.to_str());
        */
    
    //String::from(filename)
    //filename

    // absolute full path 
    //std::fs::canonicalize(filename)
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
