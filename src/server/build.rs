use std::net::{ToSocketAddrs, TcpListener};
use std::io;
use std::collections::HashMap;
use std::sync::Arc;

use crate::server::HttpServer;
use crate::server::pool::ThreadPool;
use crate::uri::Uri;
use crate::server::{Router, Responder};
use crate::request::Request;
use crate::response::Response;
use crate::error::Result;


pub struct Builder {
    verbose: bool,
    workers: usize,
    routes: HashMap<String, Responder<String>>,
    error_handler: Option<Responder<String>>,
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
    pub fn route(mut self, path: &str, handler: Responder<String>) -> Self 
    {
        self.routes.insert(path.to_string(), handler);
        self
    }

    /// Sets the request error handler.
    #[inline]
    pub fn route_err(self, handler: Responder<String>) -> Self {
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
        let error_handler = self.error_handler.unwrap_or(default_error_handler);
        let router = Arc::new(Router::from(routes, error_handler));
        Ok(HttpServer{ verbose: self.verbose, listener, pool, router })
    }

}


/// Routes request to 404.
fn default_error_handler<T>(_: &Request<T>) -> Result<Response<String>> {
    Response::builder()
        .with_status(404)
        .with_header(b"Content-Type", b"text/plain")
        .with_body("Default Error Message!".to_string())
}
