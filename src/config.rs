use structopt::StructOpt;

use crate::bookmark_query::{BookmarkQuery, BookmarkUpdateQuery};

#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(subcommand)]
    pub sub_cmd: SubOpt,

    ///Add verbosity to output
    #[structopt(long)]
    pub verbose: bool,
}

#[derive(Debug, StructOpt)]
pub enum SubOpt {
    Insert {
        #[structopt(long, short)]
        name: Option<String>,
        #[structopt(long, short)]
        url: String,
    },
    #[structopt(alias = "rm")]
    Remove {
        #[structopt(flatten)]
        query: BookmarkQuery,
    },
    #[structopt(alias = "ls")]
    List {
        #[structopt(flatten)]
        query: BookmarkQuery,
    },
    Update {
        #[structopt(flatten)]
        query: BookmarkUpdateQuery,
    },
}
