use crate::{bookmark::Bookmark, bookmark_query::BookmarkQuery, data::Data, error::Result};

/// Insert commands
/// Adds the bookmark to the data list and increments the last id
pub fn insert(url: String, mut data: Data, name: Option<String>) -> Result<()> {
    let name = Bookmark::generate_name(&url, name);

    let bookmark = Bookmark::generate_bookmark(data.last_id + 1, url, name);
    data.add_new_bookmark(bookmark);

    data.write_to_file()
}

/// List command
/// List all the bookmarks if no name flag was provided
/// List bookmarks that match the regex provided in name flag
pub fn list(data: &Data, query: BookmarkQuery) -> Result<Vec<&Bookmark>> {
    let id = query.id;
    let name = query.name;

    if let Some(id) = id {
        if let Some(b) = data.get_bookmark(id) {
            return Ok(vec![b]);
        }

        return Ok(vec![]);
    }
    let name = name.unwrap_or_else(|| "".to_owned());
    data.filter_bookmark(name)
}

/// Remove command
/// Exits if bookmark with the said id is not available
/// Remove the bookmark with the given id and exit.
pub fn remove(data: &mut Data, query: BookmarkQuery) -> Result<Vec<Bookmark>> {
    let id = query.id;
    let name = query.name;

    let mut removed = vec![];

    if let Some(name) = name {
        let i = data.remove_with_regex_name(name)?;
        for a in i {
            removed.push(a);
        }
    };
    if let Some(id) = id {
        if let Some(b) = data.remove_with_id(id) {
            removed.push(b)
        }
    };

    data.write_to_file()?;
    Ok(removed)
}
