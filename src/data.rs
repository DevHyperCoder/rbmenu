use chrono::prelude::Local;
use colored::*;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use substring::Substring;

use super::parser::{get_domain_name, is_url};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    pub is_file: bool,
    pub link: String,
    pub name: String,
    pub date: String,
    pub id: u32,
}

impl Bookmark {
    /// Generate a suitable name for Bookmark
    /// If name is empty or not provided, link is parsed to get the domain name.
    /// If name contains spaces, it is converted to underscores
    pub fn generate_name(link: &String, name: Option<String>) -> String {
        let mut name = name.unwrap_or("".to_owned());

        // If name is not provided, use the domain name
        // If provided, replace ' ' with '_'
        if name == "" {
            let m = get_domain_name(&link);
            name = link.substring(m.start(), m.end()).to_owned();
        } else {
            name = name.replace(' ', "_");
        }

        return name;
    }

    /// Return bookmark with values
    pub fn generate_bookmark(id: u32, link: String, name: String) -> Bookmark {
        Bookmark {
            is_file: !is_url(&link),
            link: link,
            name: name,
            date: Local::now().to_string(),
            id: id,
        }
    }

    /// Print a coloured output
    /// id -> yellow bold
    /// name -> cyan bold
    /// link -> blue
    pub fn colored_fmt(&self) {
        println!(
            "{} {} {}",
            self.id.to_string().yellow().bold(),
            self.name.cyan().bold(),
            self.link.blue()
        );
    }
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.name, self.link)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub bookmarks: Vec<Bookmark>,
    pub last_id: u32,
}

impl Data {
    /// Prettify the json and write to file
    pub fn write_to_file(&self) {
        fs::write(
            get_data_file_path(),
            serde_json::to_string_pretty(&self).unwrap(),
        )
        .unwrap();
    }
}

/// Create data directory and data file.data
/// Write a barebones JSON to the data file
pub fn create_data_file() {
    let data_dir = home_dir().unwrap().join(".local/share/rbmenu/");
    let data_file = data_dir.join("bookmark.json");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).unwrap();
    }

    if !data_file.exists() {
        fs::File::create(&data_file).unwrap();
    }

    let data = Data {
        bookmarks: vec![],
        last_id: 0,
    };

    fs::write(data_file, serde_json::to_string_pretty(&data).unwrap()).unwrap();
}

/// Read and parse data file into Data struct
pub fn read_data_file() -> Data {
    let data_file = get_data_file_path();

    if !data_file.exists() {
        create_data_file();
    }

    let content = fs::read_to_string(data_file).expect("asdf");
    serde_json::from_str(&content).unwrap()
}

/// Return data file path
pub fn get_data_file_path() -> std::path::PathBuf {
    home_dir()
        .unwrap()
        .join(".local/share/rbmenu/bookmark.json")
}
