use super::config::Config;
use super::data::get_data_file_path;
use super::data::{Bookmark, Data};
use super::parser::is_url;
use regex::Regex;
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

    data.bookmarks.push(bookmark);

    fs::write(
        get_data_file_path(),
        serde_json::to_string_pretty(&data).unwrap(),
    )
    .unwrap();
}

pub fn list(input: String, data: Data, _config: Config) {
    // No input
    if input.len() == 0 {
        print_bookmark(&data.bookmarks);
        return;
    }

    let mut search_results = vec![];

    // Match Regex
    for i in data.bookmarks {
        if Regex::new(&input).unwrap().is_match(&i.name) {
            search_results.push(i);
        }
    }

    print_bookmark(&search_results);
}

fn print_bookmark(input: &Vec<Bookmark>) {
    for i in input {
        println!("{}", i);
    }
}
