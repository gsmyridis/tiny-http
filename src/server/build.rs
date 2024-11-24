use std::collections::HashMap;
use std::io;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::Arc;

use super::RequestHandler;
use crate::http::{Method, Uri};
use crate::server::{pool::ThreadPool, HttpServer, Router};

pub struct Builder<T> {
    workers: usize,
    routes: HashMap<(Uri, Method), RequestHandler<T>>,
    error_handler: Option<RequestHandler<T>>,
}

impl<T> Default for Builder<T> {
    fn default() -> Self {
        Builder {
            workers: 1,
            routes: HashMap::new(),
            error_handler: None,
        }
    }
}

impl<T> Builder<T> {
    /// Specifies the size of the thread pool for the Server that is constructed.
    #[inline]
    pub fn workers(self, size: usize) -> Self {
        Builder {
            workers: size,
            ..self
        }
    }

    /// Adds a route.
    ///
    /// A route is a request path and the corresponding functions that handles
    /// the request.
    #[inline]
    pub fn route(mut self, path: &str, method: Method, handler: RequestHandler<T>) -> Self {
        self.routes.insert((Uri::new(path), method), handler);
        self
    }

    /// Sets the request error handler.
    #[inline]
    pub fn route_err(self, handler: RequestHandler<T>) -> Self {
        Builder {
            error_handler: Some(handler),
            ..self
        }
    }

    /// Consumes the builder and returns an HTTP server that listens to the specified
    /// address.
    pub fn bind<A: ToSocketAddrs>(self, addr: A) -> io::Result<HttpServer<T>> {
        let listener = TcpListener::bind(addr)?;
        let pool = ThreadPool::new(self.workers);
        let error_handler = self
            .error_handler
            .expect("Error handler must be specified.");
        let router = Arc::new(Router::from(self.routes, error_handler));
        Ok(HttpServer {
            listener,
            pool,
            router,
        })
    }
}
