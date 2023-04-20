use std::{error::Error, fs, path::PathBuf};

pub fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

pub fn read_file_contents(path: PathBuf) -> Result<u64, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(strip_trailing_newline(&contents).parse()?)
}

#[cfg(test)]
mod tests {
    use super::strip_trailing_newline;

    #[test]
    fn test_strip_trailing_newline() {
        assert_eq!(strip_trailing_newline(""), "");
        assert_eq!(strip_trailing_newline("0"), "0");
        assert_eq!(strip_trailing_newline("0\n"), "0");
        assert_eq!(strip_trailing_newline("0\r\n"), "0");
        assert_eq!(strip_trailing_newline("0\n\n"), "0\n");
        assert_eq!(strip_trailing_newline("0\r\n\r\n"), "0\r\n");
    }
}
