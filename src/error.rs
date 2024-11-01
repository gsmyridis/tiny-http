use std::result;

use crate::method::InvalidMethod;
use crate::version::InvalidVersion;
use crate::uri::InvalidUri;
use crate::status::InvalidStatusCode;


#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}


pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
enum ErrorKind {
    Method(InvalidMethod),
    Version(InvalidVersion),
    Uri(InvalidUri),
    StatusCode(InvalidStatusCode),
}


impl From<InvalidMethod> for Error {
    fn from(err: InvalidMethod) -> Error {
        Error{ inner: ErrorKind::Method(err) }
    }
}


impl From<InvalidVersion> for Error {
    fn from(err: InvalidVersion) -> Error {
        Error{ inner: ErrorKind::Version(err) }
    }
}


impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error{ inner: ErrorKind::Uri(err) }
    }
}


impl From<InvalidStatusCode> for Error {
    fn from(err: InvalidStatusCode) -> Error {
        Error{ inner: ErrorKind::StatusCode(err) }
    }
}
