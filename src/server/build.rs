use std::collections::HashMap;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::Arc;

use super::RequestHandler;
use crate::error::{Error, FailedConnection, NoErrorHandler, Result};
use crate::http::{Method, Uri};
use crate::server::{pool::ThreadPool, HttpServer, Router};

pub struct Builder<T> {
    inner: Result<Parts<T>>,
}

impl<T> Default for Builder<T> {
    #[inline]
    fn default() -> Self {
        Builder {
            inner: Ok(Parts::default()),
        }
    }
}

impl<T> Builder<T> {
    /// Specifies the size of the thread pool for the Server that is constructed.
    #[inline]
    pub fn workers(self, size: usize) -> Self {
        let inner = self.inner.and_then(move |mut parts| {
            parts.workers = size;
            Ok(parts)
        });
        Self { inner }
    }

    /// Adds a route.
    ///
    /// A route is a request path and the corresponding functions that handles
    /// the request.
    #[inline]
    pub fn route<P, M>(self, path: P, method: M, handler: RequestHandler<T>) -> Self
    where
        Uri: TryFrom<P>,
        Method: TryFrom<M>,
        <Uri as TryFrom<P>>::Error: Into<Error>,
        <Method as TryFrom<M>>::Error: Into<Error>,
    {
        let inner = self.inner.and_then(move |mut parts| {
            let uri = TryFrom::try_from(path).map_err(Into::into)?;
            let method = TryFrom::try_from(method).map_err(Into::into)?;
            parts.routes.insert((uri, method), handler);
            Ok(parts)
        });
        Self { inner }
    }

    /// Sets the request error handler.
    #[inline]
    pub fn route_err(self, handler: RequestHandler<T>) -> Self {
        let inner = self.inner.and_then(move |mut parts| {
            parts.error_handler = Some(handler);
            Ok(parts)
        });
        Self { inner }
    }

    /// Consumes the builder and returns an HTTP server that listens to the specified
    /// address.
    pub fn bind<A: ToSocketAddrs>(self, addr: A) -> Result<HttpServer<T>> {
        let listener = TcpListener::bind(addr).or(Err(Error::from(FailedConnection)))?;
        let (pool, router) = self.inner.and_then(move |parts| {
            let pool = ThreadPool::new(parts.workers);
            let error_handler = parts.error_handler.ok_or(Error::from(NoErrorHandler))?;
            let router = Arc::new(Router::from(parts.routes, error_handler));
            Ok((pool, router))
        })?;

        Ok(HttpServer {
            listener,
            pool,
            router,
        })
    }
}

struct Parts<T> {
    workers: usize,
    routes: HashMap<(Uri, Method), RequestHandler<T>>,
    error_handler: Option<RequestHandler<T>>,
}

impl<T> Default for Parts<T> {
    #[inline]
    fn default() -> Self {
        Parts {
            workers: 1,
            routes: HashMap::new(),
            error_handler: None,
        }
    }
}
