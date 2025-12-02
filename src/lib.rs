use std::fs;

pub fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
}
