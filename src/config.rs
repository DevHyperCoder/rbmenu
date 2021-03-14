use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    ///Insert a new bookmark.
    #[structopt(short)]
    pub insert: bool,

    ///List all  bookmarks.
    #[structopt(short)]
    pub list: bool,

    ///Add verbosity to output
    #[structopt(long)]
    pub verbose: bool,

    ///Name of bookmark ; Required for -i or -l
    #[structopt(short, long)]
    pub name: Option<String>,
}
