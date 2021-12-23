use home::home_dir;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::bookmark::Bookmark;

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

    pub fn add_new_bookmark(&mut self, bookmark: Bookmark) {
        self.bookmarks.push(bookmark);
        self.last_id += 1;
    }

    // TODO refactor to a hashmap prolly
    pub fn get_bookmark(&self, id: u32) -> Option<&Bookmark> {
        let mut bookmark = None;
        for b in &self.bookmarks {
            if b.id == id {
                bookmark = Some(b);
                break;
            }
        }
        bookmark
    }

    pub fn filter_bookmark(&self, name: String) -> Vec<&Bookmark> {
        self.bookmarks
            .iter()
            .filter(|b| Regex::new(&name).unwrap().is_match(&b.name))
            .collect::<Vec<&Bookmark>>()

    }

    pub fn remove_with_regex_name(&mut self, name: String) -> Vec<Bookmark> {
        self
            .bookmarks
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_i, b)| Regex::new(&name).unwrap().is_match(&b.name))
            .map(|(i, b)| {
                self.bookmarks.remove(i);
                b
            })
            .collect::<Vec<Bookmark>>()

        
    }

    pub fn remove_with_id(&mut self, id: u32) -> Vec<Bookmark> {
        let things_to_remove = &self
            .bookmarks
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_i, b)| b.id == id)
            .map(|(i, b)| {
                self.bookmarks.remove(i);
                b
            })
            .collect::<Vec<Bookmark>>();

        things_to_remove.clone()
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
