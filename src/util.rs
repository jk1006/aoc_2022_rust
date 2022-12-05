use std::fs::read_to_string;
pub fn split_input_empty_lines(input_file: &str) -> Vec<String> {
    return read_to_string(input_file)
        .unwrap_or_else(|_| panic!("unable to open file"))
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
}
