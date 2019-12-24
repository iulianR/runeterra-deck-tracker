use core::fmt;
use serde::export::Formatter;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    ResponseParse(serde_json::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Http(ref err) => reqwest::Error::fmt(err, f),
            Error::ResponseParse(ref err) => serde_json::error::Error::fmt(err, f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref err) => err.description(),
            Error::ResponseParse(ref err) => err.description(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::ResponseParse(err)
    }
}
