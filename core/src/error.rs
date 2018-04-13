use std::io;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    Str(String),
    AtomCreate,
    IoError(io::Error),
    Utf8(Utf8Error)
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Str(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
