use std::error;
use std::fmt;
use std::io;
use std::result;
use std::str;
use std::string;

#[derive(Debug)]
pub enum HciErrorKind {
    NotEnoughData,
    NotFound,
    InvalidValue,
    InvalidLength,
}

#[derive(Debug)]
pub struct HciError {
    pub kind: HciErrorKind,
}

impl HciError {
    pub fn new(kind: HciErrorKind) -> HciError {
        HciError { kind }
    }
}

impl fmt::Display for HciError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NetlinkError {:?}", self.kind)
    }
}

impl error::Error for HciError {
    fn description(&self) -> &str {
        "HciError"
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(str::Utf8Error),
    FromUtf8(string::FromUtf8Error),
    Hci(HciError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Utf8(ref err) => write!(f, "UTF8 error: {}", err),
            Error::FromUtf8(ref err) => write!(f, "From UTF8 error: {}", err),
            Error::Hci(ref err) => write!(f, "HCI error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Utf8(ref err) => err.description(),
            Error::FromUtf8(ref err) => err.description(),
            Error::Hci(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            Error::Hci(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<HciError> for Error {
    fn from(err: HciError) -> Error {
        Error::Hci(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
