mod commands;
mod config;

use config::Config;

use commands::{insert, list, view};
use structopt::StructOpt;

pub fn run() {
    let opts = Config::from_args();

    if opts.insert {
        insert(opts);
    } else if opts.list {
        list(opts);
    } else if opts.view {
        view(opts);
    }
}
