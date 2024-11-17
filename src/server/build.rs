use std::net::{ToSocketAddrs, TcpListener};
use std::io;
use std::collections::HashMap;
use std::sync::Arc;

use crate::server::HttpServer;
use crate::server::pool::ThreadPool;
use crate::uri::Uri;
use crate::server::{Router, RequestHandler};


pub struct Builder {
    verbose: bool,
    workers: usize,
    routes: HashMap<String, RequestHandler<String>>,
    error_handler: Option<RequestHandler<String>>,
}

impl Default for Builder {
    fn default() -> Self {
        Builder{  verbose: false, workers: 1, routes: HashMap::new(), error_handler: None }
    }
}

impl Builder {

    /// Turns on/off the verbose mode.
    #[inline]
    pub fn verbose(self, verbose: bool) -> Self {
        Builder{ verbose, ..self}
    }

    /// Specifies the size of the thread pool for the Server that is constructed.
    #[inline]
    pub fn workers(self, size: usize) -> Self {
        Builder{ workers: size, ..self }
    }

    /// Adds a route. 
    ///
    /// A route is a request path and the corresponding functions that handles
    /// the request.
    #[inline]
    pub fn route(mut self, path: &str, handler: RequestHandler<String>) -> Self 
    {
        self.routes.insert(path.to_string(), handler);
        self
    }

    /// Sets the request error handler.
    #[inline]
    pub fn route_err(self, handler: RequestHandler<String>) -> Self {
        Builder{ error_handler: Some(handler), ..self }
    }

    /// Consumes the builder and returns an HTTP server that listens to the specified
    /// address.
    pub fn bind<A: ToSocketAddrs>(self, addr: A) -> io::Result<HttpServer> {
        let listener = TcpListener::bind(addr)?;
        let pool = ThreadPool::new(self.workers);
        let routes = self.routes.into_iter()
            .map(|(path, fun)| {
                let uri = Uri::new(&path);
                (uri, fun)
            })
            .collect::<HashMap<_, _>>();
        let router = Arc::new(Router::from(routes, self.error_handler));
        Ok(HttpServer{ verbose: self.verbose, listener, pool, router })
    }

}

