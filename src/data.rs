use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    pub is_file: bool,
    pub link: String,
    pub name: String,
    pub date: String,
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.link)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub bookmarks: Vec<Bookmark>,
}

pub fn create_data_file() {
    let data_dir = home_dir().unwrap().join(".local/share/rbmenu/");
    let data_file = data_dir.join("bookmark.json");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).unwrap();
    }

    if !data_file.exists() {
        fs::File::create(&data_file).unwrap();
    }

    let data = Data { bookmarks: vec![] };

    fs::write(data_file, serde_json::to_string_pretty(&data).unwrap()).unwrap();
}

pub fn read_data_file() -> Data {
    let data_file = get_data_file_path();

    if !data_file.exists() {
        create_data_file();
    }

    let content = fs::read_to_string(data_file).expect("asdf");
    serde_json::from_str(&content).unwrap()
}

pub fn get_data_file_path() -> std::path::PathBuf {
    home_dir()
        .unwrap()
        .join(".local/share/rbmenu/bookmark.json")
}
