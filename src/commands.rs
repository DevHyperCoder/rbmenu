use super::config::Config;
use super::data::get_data_file_path;
use super::data::{Bookmark, Data};
use super::parser::*;
use chrono::prelude::Local;
use regex::Regex;
use std::fs;
use substring::Substring;

fn generate_name(link: &String, config: Config) -> String {
    let mut name = config.name.unwrap_or("".to_owned());

    // If name is not provided, use the domain name
    // If provided, replace ' ' with '_'
    if name == "" {
        let m = get_name(&link);
        name = link.substring(m.start(), m.end()).to_owned();
    } else {
        name = name.replace(' ', "_");
    }

    return name;
}

fn generate_bookmark(id:u32,link: String, name: String) -> Bookmark {
    Bookmark {
        is_file: !is_url(&link),
        link: link,
        name: name,
        date: Local::now().to_string(),
        id: id,
    }
}

pub fn insert(input: String, mut data: Data, config: Config) {
    let name = generate_name(&input, config);

    let bookmark = generate_bookmark(data.last_id+1, input, name);
    data.bookmarks.push(bookmark);
    data.last_id += 1;

    fs::write(
        get_data_file_path(),
        serde_json::to_string_pretty(&data).unwrap(),
    )
    .unwrap();
}

pub fn list(data: Data, config: Config) {
    let name = config.name.unwrap_or("".to_owned());

    // No input
    if name.len() == 0 {
        print_bookmark(&data.bookmarks);
        return;
    }

    let mut search_results = vec![];

    // Match Regex
    for i in data.bookmarks {
        if Regex::new(&name).unwrap().is_match(&i.name) {
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
