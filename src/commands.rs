use super::config::Config;
use super::parser::is_url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Bookmark {
    is_file: bool,
    link: String,
    name: String,
}

pub fn insert(input: String, config: Config) {
    let name = match config.name {
        None => "".to_owned(),
        Some(i) => i,
    };
    let bookmark = Bookmark {
        is_file: !is_url(&input),
        link: input,
        name: name,
    };
    let json = serde_json::to_string(&bookmark);
    println!("{}", json.unwrap());
}

pub fn view(_input: String, _config: Config) {
    println!("TODO - view");
}

pub fn list(_input: String, _config: Config) {
    println!("TODO - list");
}
