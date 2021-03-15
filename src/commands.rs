use super::config::Config;
use super::data::{Bookmark, Data};

use regex::Regex;

/// Insert commands
/// Adds the bookmark to the data list and increments the last id
pub fn insert(input: String, mut data: Data, config: Config) {
    let name = Bookmark::generate_name(&input, config.name);

    let bookmark = Bookmark::generate_bookmark(data.last_id + 1, input, name);
    data.bookmarks.push(bookmark);
    data.last_id += 1;

    data.write_to_file();
}

/// List command
/// List all the bookmarks if no name flag was provided
/// List bookmarks that match the regex provided in name flag
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

/// Print all bookmarks in the vector
fn print_bookmark(input: &Vec<Bookmark>) {
    for i in input {
        println!("{}", i);
    }
}
