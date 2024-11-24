pub mod parts;
pub mod build;

use crate::version::Version;
use crate::status::StatusCode;
use crate::header::HeaderMap;

use parts::Parts;
use build::Builder;

use std::fmt;


pub struct Response<T> {
    head: Parts,
    body: T
}


impl Response<()> {
    
    /// Returns a `Builder` that constructs a `Response`.
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}


impl<T> Response<T> {

    /// Returns a reference to the HTTP version of the `Response`.
    #[inline]
    pub fn version(&self) -> &Version {
        &self.head.version
    }

    /// Returns a reference to the status code of the `Response`.
    #[inline]
    pub fn status(&self) -> &StatusCode {
        &self.head.status
    }

    /// Returns a reference to the body of the `Response`.
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Returns a reference to the header-map of the `Response`.
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.head.headers
    }
}


impl<T: fmt::Display> fmt::Display for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\r\n", 
            self.version(), 
            self.status().code(), 
            self.status().msg().expect("Guaranteed by construction."))?;
        for (name, val) in self.headers() {
            write!(f, "{}: {}\r\n", name, val)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}
