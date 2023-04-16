use std::fs;

pub fn parse_html(path: &str) -> String {
    let mut absolute_path = std::env::current_dir()
        .expect("Could not get path");
    absolute_path.push("static");
    absolute_path.push(path);
    match fs::read_to_string(absolute_path) {
        Ok(v) => v,
        Err(..) => "".to_string(),
    }
}
