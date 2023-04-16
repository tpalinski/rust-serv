use std::fs;

pub fn parse_html(path: &str) -> String {
    let mut absolute_path = std::env::current_dir()
        .expect("Could not get path");
    absolute_path.push("static");
    absolute_path.push(path);
    fs::read_to_string(absolute_path).unwrap()
}
