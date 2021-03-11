mod commands;
mod config;
mod parser;

use config::Config;

use commands::{insert, list, view};
use std::io::stdin;
use structopt::StructOpt;

pub fn run() {
    let opts = Config::from_args();

    let mut input_url = String::new();
    stdin().read_line(&mut input_url).unwrap();

    input_url = input_url.trim().to_owned();

    // Run based on options
    if opts.insert {
        insert(input_url, opts);
    } else if opts.list {
        list(input_url, opts);
    } else if opts.view {
        view(input_url, opts);
    }
}
