use super::data::{Bookmark, Data};
use regex::Regex;

/// Insert commands
/// Adds the bookmark to the data list and increments the last id
pub fn insert(input: String, mut data: Data, name: Option<String>) {
    let name = Bookmark::generate_name(&input, name);

    let bookmark = Bookmark::generate_bookmark(data.last_id + 1, input, name);
    data.bookmarks.push(bookmark);
    data.last_id += 1;

    data.write_to_file();
}

/// List command
/// List all the bookmarks if no name flag was provided
/// List bookmarks that match the regex provided in name flag
pub fn list(data: Data, name: Option<String>) -> Vec<Bookmark> {
    let name = name.unwrap_or_else(|| "".to_owned());

    // No input
    if name.is_empty() {
        data.bookmarks
    } else {
        let mut search_results = vec![];

        // Match Regex
        for i in data.bookmarks {
            if Regex::new(&name).unwrap().is_match(&i.name) {
                search_results.push(i);
            }
        }
        search_results
    }
}

/// Remove command
/// Exits if bookmark with the said id is not available
/// Remove the bookmark with the given id and exit.
pub fn remove(data: &mut Data, index: Option<u32>, _name: Option<String>) {
    // Loop through bookmarks using a index so we can remove it later.
    for i in 0..data.bookmarks.len() {
        let bookmark = &data.bookmarks[i];

        if let Some(index) = index {
            if index == bookmark.id {
                data.bookmarks.remove(i);
            }
        }

        //if name.is_some() {
        //// TODO FIX
        //if name.as_ref().unwrap() == &bookmark.name {
        //data.bookmarks.remove(i);
        //}
        //}
    }
    data.write_to_file();
}

// Print all bookmarks in the vector (in color)
//fn print_bookmark(input: Vec<&Bookmark>) {
//for i in input {
//i.colored_fmt();
//}
//}
