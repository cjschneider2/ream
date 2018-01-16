
#[derive(Debug)]
pub enum Error {
    Str(String),
    AtomCreate,
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Str(err)
    }
}
