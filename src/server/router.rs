use std::collections::HashMap;

use crate::uri::Uri;
use crate::request::Request;
use crate::response::Response;
use crate::error::Result;
use super::RequestHandler;


pub struct Router<T> {
    routes: HashMap<Uri, RequestHandler<T>>,
    error_handler: RequestHandler<T>,
}


impl<T> Router<T> {

    /// Creates a new `Router` from a map between paths and request handles.
    pub fn from(routes: HashMap<Uri, RequestHandler<T>>, error_handler: RequestHandler<T>) -> Self {
        Router{ routes, error_handler }
    }

    /// Handles the request.
    ///
    /// The request is hanled based on the specified routes. In case of an error
    /// the request is handled with the `Router`'s specified error handler.
    pub fn handle_request(&self, request: &Request<T>) -> Result<Response<T>> {
        match self.get_handler(request.uri()) {
            Some(handler) => handler(request),
            None => self.handle_error(request)
        }
    }

    /// Handle request in case of error.
    pub fn handle_error(&self, request: &Request<T>) -> Result<Response<T>> {
        (self.error_handler)(request)
    }

    /// Gets the function that handles the request, for given path.
    ///
    /// The request handler
    /// Paths are expected to start with the prefix '/'.
    fn get_handler(&self, path: &Uri) -> Option<&RequestHandler<T>> {
        let mut handlers = Vec::new();

        if path == "/" {
            if let Some(root_handler) = self.routes.get(path) {
                handlers.push(root_handler);
            }
        } else {
            for (p, handler) in self.routes.iter() {
                if path.inner.starts_with(&p.inner) && (p != "/") {
                    handlers.push(handler);
                } 
            }
        }

        match handlers.len() {
            0 => None,
            1 => Some(handlers[0]),
            // TODO: Maybe change the return type to Result with two reasons.
            _ => panic!("There are more than one paths matching the pattern.")
        }
    }
}

