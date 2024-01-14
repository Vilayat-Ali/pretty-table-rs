use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Input Error: {0}")]
    InputError(&'static str),
}
