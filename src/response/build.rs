use crate::response::parts::Parts;
use crate::error::{Error, Result};
use crate::status::StatusCode;
use crate::version::Version;
use crate::response::Response;

use std::convert::TryFrom;

/// An HTTP response builder
///
/// This type can be used to construct an instance of `Response` through a builder-like pattern.
pub struct Builder {
    pub inner: Result<Parts>,
}


impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Builder {
            inner: Ok(Parts::new())
        }
    }
}


impl Builder {
    
    /// Creates a new default instance of `Builder` to construct a `Response`.
    #[inline]
    pub fn new() -> Self {
        Builder::default()
    }

    /// Sets the status code of the response that the `Builder` is constructing.
    pub fn with_status<T>(self, status: T) -> Self 
        where
            StatusCode: TryFrom<T>,
            <StatusCode as TryFrom<T>>::Error: Into<Error> 
    {
        let inner = self.inner.and_then(move |mut head| {
            let status_code = TryFrom::try_from(status).map_err(Into::into)?;
            head.status = status_code;
            Ok(head)
        });

        Builder{inner}
    }

    /// Sets the HTTP version of the response that the `Builder` is constructing.
    pub fn with_version<T>(self, version: T) -> Self 
        where
            Version: TryFrom<T>,
            <Version as TryFrom<T>>::Error: Into<Error>
    {
        let inner = self.inner.and_then(move |mut head| {
            let version = TryFrom::try_from(version).map_err(Into::into)?;
            head.version = version;
            Ok(head)
        });

        Builder { inner }
    }

    /// Sets the body of the response that the `Builder` is constructing.
    pub fn with_body<T>(self, body: T) -> Result<Response<T>> {
        self.inner.map(move |head| Response { head, body })
    }

}
