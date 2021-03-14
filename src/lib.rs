mod commands;
mod config;
mod data;
mod parser;

use commands::{insert, list};
use config::Config;
use data::read_data_file;

use std::io::stdin;
use structopt::StructOpt;

pub fn run() {
    let opts = Config::from_args();
    let data = read_data_file();

    // Run based on options
    if opts.insert {
        // Read from stdin
        let mut input_url = String::new();
        stdin().read_line(&mut input_url).unwrap();
        input_url = input_url.trim().to_owned();

        insert(input_url, data, opts);
    } else if opts.list {
        list(data, opts);
    }
}
