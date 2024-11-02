pub mod parts;
pub mod build;

use crate::version::Version;
use crate::status::StatusCode;

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
}


impl fmt::Display for Response<String>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}\r\nContent-Length: {}\r\n\r\n{}", 
            self.version(), 
            self.body().len(), 
            self.body())
        )

    }
}
