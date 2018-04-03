use std::io;

#[derive(Debug)]
pub enum Error {
    Str(String),
    AtomCreate,
    IoError(io::Error),
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Str(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
