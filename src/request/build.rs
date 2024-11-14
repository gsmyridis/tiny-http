use crate::request::parts::Parts;
use crate::error::{Result, Error};
use crate::method::Method;
use crate::version::Version;
use crate::uri::Uri;
use crate::request::Request;
use crate::body::Body;
use crate::header::name::HeaderName;
use crate::header::value::HeaderValue;


/// An HTTP request builder
///
/// This type can be used to construct an instance or `Request` through a builder-like pattern.
pub struct Builder {
    pub inner: Result<Parts>
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

    /// Creates a new default instance of `Builder` to construct either a `Request`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the HTTP method of the request that the `Builder` is constructing.
    pub fn with_method<T>(self, method: T) -> Self 
        where 
            Method: TryFrom<T>,
            <Method as TryFrom<T>>::Error: Into<Error>
    {
        let inner = self.inner.and_then(move |mut head| {
            let method = TryFrom::try_from(method).map_err(Into::into)?;
            head.method = method;
            Ok(head)
        });
        Builder{ inner }
    }

    /// Sets the URI of the request that the `Builder` is constructing.
    pub fn with_uri<T>(self, uri: T) -> Self 
        where
            Uri: TryFrom<T>,
            <Uri as TryFrom<T>>::Error: Into<Error>
    {
        let inner = self.inner.and_then(move |mut head| {
            let uri = TryFrom::try_from(uri).map_err(Into::into)?;
            head.uri = uri;
            Ok(head)
        });
        Builder{inner}
    }


    /// Sets the HTTP version of the request that the `Builder` is constructing.
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
        Builder{ inner }
    }

    /// Inserts a pair of header-name and header-value to the `HeaderMap`.
    pub fn with_header<N, V>(self, name: N, val: V) -> Self 
        where
            HeaderValue: From<V>,
            HeaderName: TryFrom<N>,
            <HeaderName as TryFrom<N>>::Error: Into<Error>,
    {
        let inner = self.inner.and_then(move |mut head| {
            head.headers.insert(name, val)?;
            Ok(head)
        });

        Builder{inner}
    }

    /// Sets the body of the request that the `Builder` is constructing.
    pub fn with_body<T: Body>(self, body: T) -> Result<Request<T>> {
        // let len = body.content_len();
        // let builder = self.with_header(b"Content-Length", b"{len}");
        self.inner.map(move |head| Request { head, body }) 
    }
}
