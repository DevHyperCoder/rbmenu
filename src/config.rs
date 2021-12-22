use structopt::StructOpt;

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
    Remove {
        #[structopt(long, short)]
        id: Option<u32>,
        #[structopt(long, short)]
        name: Option<String>,
    },
    List {
        #[structopt(long, short)]
        name: Option<String>,
    },
}
