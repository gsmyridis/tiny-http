use std::net::{ToSocketAddrs, TcpListener};
use std::io;
use std::collections::HashMap;
use std::sync::Arc;

use crate::server::HttpServer;
use crate::server::pool::ThreadPool;
use crate::uri::Uri;
use crate::method::Method;
use crate::server::Router;
use super::RequestHandler;


pub struct Builder<T> {
    verbose: bool,
    workers: usize,
    routes: HashMap<(String, Method), RequestHandler<T>>,
    error_handler: Option<RequestHandler<T>>,
}

impl<T> Default for Builder<T> {
    fn default() -> Self {
        Builder{  verbose: false, workers: 1, routes: HashMap::new(), error_handler: None }
    }
}

impl<T> Builder<T> {

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
    pub fn route(mut self, path: &str, method: Method, handler: RequestHandler<T>) -> Self 
    {
        // If the URI is not in the routes, create a hashmap <method, handelr> and add current one.
        
        // If the URI is there, then insert to the existing Uri
        self.routes.insert((path.to_string(), method), handler);
        self
    }

    /// Sets the request error handler.
    #[inline]
    pub fn route_err(self, handler: RequestHandler<T>) -> Self {
        Builder{ error_handler: Some(handler), ..self }
    }

    /// Consumes the builder and returns an HTTP server that listens to the specified
    /// address.
    pub fn bind<A: ToSocketAddrs>(self, addr: A) -> io::Result<HttpServer<T>> {
        let listener = TcpListener::bind(addr)?;
        let pool = ThreadPool::new(self.workers);
        let routes = self.routes.into_iter()
            .map(|((path, method), fun)| {
                let uri = Uri::new(&path);
                ((uri, method), fun)
            })
            .collect::<HashMap<_, _>>();
        let error_handler = self.error_handler.unwrap();
        // let error_handler = self.error_handler.unwrap_or(|| { 
        //         Response::builder()
        //             .with_status(404)
        //             .with_header(b"Content-Type", b"text/plain")
        //             .with_body("Default Error Message!".to_string())
        //     });
        let router = Arc::new(Router::from(routes, error_handler));
        Ok(HttpServer{ verbose: self.verbose, listener, pool, router })
    }

}
