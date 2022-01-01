pub mod bookmark;
pub mod bookmark_query;
pub mod commands;
pub mod config;
pub mod data;
pub mod error;
pub mod parser;

use commands::{insert, list, remove, update};
use config::{Config, SubOpt};
use data::read_data_file;

use error::Result;
use structopt::StructOpt;

/// Call command functions based on given options
pub fn run() -> Result<()> {
    let opts = Config::from_args();
    let mut data = read_data_file()?;

    match opts.sub_cmd {
        SubOpt::Insert { name, url } => insert(url, data, name)?,
        SubOpt::Remove { query } => {
            let removed = remove(&mut data, query)?;
            if removed.is_empty() {
                println!("Nothing to remove!");
                return Ok(());
            }
            println!("Removed: ");
            for i in removed {
                i.colored_fmt()
            }
        }
        SubOpt::List { query } => {
            let listed = list(&data, query)?;
            for i in listed {
                i.colored_fmt()
            }
        }
        SubOpt::Update { query } => update(&mut data, query)?,
    }
    Ok(())
}
