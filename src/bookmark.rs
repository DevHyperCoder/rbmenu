use crate::parser::{get_domain_name, is_url};
use chrono::prelude::Local;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use substring::Substring;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn generate_name(link: &str, name: Option<String>) -> String {
        let mut name = name.unwrap_or_else(|| "".to_owned());

        // If name is not provided, use the domain name
        // If provided, replace ' ' with '_'
        if name.is_empty() {
            let m = get_domain_name(link);
            name = link.substring(m.start(), m.end()).to_owned();
        } else {
            name = name.replace(' ', "_");
        }

        name
    }

    /// Return bookmark with values
    pub fn generate_bookmark(id: u32, link: String, name: String) -> Bookmark {
        Bookmark {
            is_file: !is_url(&link),
            link,
            name,
            date: Local::now().to_string(),
            id,
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

