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

    /// Returns a string for the response line, that is "HTTP_VERSION STATUS_CODE STATUS_MSG"
    pub fn response_line(&self) -> String {
        format!("{} {} {}", 
            self.version(), 
            self.status().code(), 
            self.status().msg().expect("It's guaranteed to have Some by construction."),
        )
    }
    
    /// Returns a string for the collected headers.
    ///
    /// The string is formated as: 
    ///
    ///     "HEADER_NAME_1: HEADER_VAL_1\r\n
    ///      HEADER_NAME_2: HEADER_VAL_2\r\n
    ///      ..
    ///      HEADER_NAME_k: HEADER_VAL_k" 
    ///
    /// The order of the headers is random.
    pub fn collect_headers(&self) -> String {
        let mut header_lines = Vec::new();
        for (name, val) in &self.headers().inner {
            header_lines.push(format!("{name}: {val}"));
        }
        header_lines.join("\r\n")
    }
}


impl<T: fmt::Display> fmt::Display for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\r\n{}\r\n\r\n{}",
            self.response_line(),
            self.collect_headers(),
            self.body()
        )
    }
}
