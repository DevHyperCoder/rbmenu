mod commands;
mod config;
mod data;
mod parser;

use commands::{insert, list, remove};
use config::{Config, SubOpt};
use data::read_data_file;

use structopt::StructOpt;

/// Call command functions based on given options
pub fn run() {
    let opts = Config::from_args();
    let mut data = read_data_file();

    match opts.sub_cmd {
        SubOpt::Insert { name, url } => insert(url, data, name),
        SubOpt::Remove { id, name } => {
            remove(&mut data, id, name);
        }
        SubOpt::List { name } => {
            let listed = list(data, name);
            for i in listed {
                i.colored_fmt()
            }
        }
    }
}
