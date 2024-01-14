use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File Error: failed to read or write to file")]
    FileError,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::FileError
    }
}
