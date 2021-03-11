use super::config::Config;
use super::data::get_data_file_path;
use super::data::{Bookmark, Data};
use super::parser::is_url;
use std::fs;

pub fn insert(input: String, mut data: Data, config: Config) {
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

    data.bookmarks.push(bookmark);
    let json = serde_json::to_string_pretty(&data);
    println!("{}", json.unwrap());

    fs::write(
        get_data_file_path(),
        serde_json::to_string_pretty(&data).unwrap(),
    )
    .unwrap();
}

pub fn view(_input: String, _data: Data, _config: Config) {
    println!("TODO - view");
}

pub fn list(_input: String, _data: Data, _config: Config) {
    println!("TODO - list");
}
