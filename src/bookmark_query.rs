use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug, Clone, StructOpt)]
pub struct BookmarkQuery {
    #[structopt(long, short)]
    pub id: Option<u32>,
    #[structopt(long, short)]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, StructOpt)]
pub struct BookmarkUpdateQuery {
    #[structopt(long, short)]
    pub id: u32,
    #[structopt(long, short)]
    pub name: Option<String>,
    #[structopt(long, short)]
    pub link: Option<String>,
}
