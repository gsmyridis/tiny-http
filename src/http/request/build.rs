use crate::error::{Error, Result};
use crate::http::{Body, HeaderName, HeaderValue, Method, Request, Uri, Version};
use super::parts::Parts;

/// An HTTP request builder
///
/// This type can be used to construct an instance or `Request` through a builder-like pattern.
pub struct Builder {
    pub inner: Result<Parts>,
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Builder {
            inner: Ok(Parts::new()),
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
        <Method as TryFrom<T>>::Error: Into<Error>,
    {
        let inner = self.inner.and_then(move |mut head| {
            let method = TryFrom::try_from(method).map_err(Into::into)?;
            head.method = method;
            Ok(head)
        });
        Builder { inner }
    }

    /// Sets the URI of the request that the `Builder` is constructing.
    pub fn with_uri<T>(self, uri: T) -> Self
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<Error>,
    {
        let inner = self.inner.and_then(move |mut head| {
            let uri = TryFrom::try_from(uri).map_err(Into::into)?;
            head.uri = uri;
            Ok(head)
        });
        Self{ inner }
    }

    /// Sets the HTTP version of the request that the `Builder` is constructing.
    pub fn with_version<T>(self, version: T) -> Self
    where
        Version: TryFrom<T>,
        <Version as TryFrom<T>>::Error: Into<Error>,
    {
        let inner = self.inner.and_then(move |mut head| {
            let version = TryFrom::try_from(version).map_err(Into::into)?;
            head.version = version;
            Ok(head)
        });
        Self{ inner }
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
        Self{ inner }
    }

    /// Sets the body of the request that the `Builder` is constructing.
    #[inline]
    pub fn with_body<T: Body>(self, body: T) -> Result<Request<T>> {
        self.inner.map(move |head| Request { head, body })
    }
}
