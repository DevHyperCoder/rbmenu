pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Unable to write to data file
    DataFileWrite,
    // Unable to prettify json
    JsonPrettify,
    // Bad Regex input
    InvalidRegex,
    // Unable to get home dir
    Home,
    // Unable to create data dir
    DataDirCreate,
    // Unable to create data file
    DataFileCreate,
    // Unable to read data file
    DataFileRead,
    // Unable to parse data file
    DataFileParse,
}

impl From<regex::Error> for Error {
    fn from(_: regex::Error) -> Self {
        Error::InvalidRegex
    }
}
