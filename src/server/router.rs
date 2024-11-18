use std::collections::HashMap;

use crate::uri::Uri;
use super::Responder;
use crate::request::Request;
use crate::response::Response;
use crate::error::Result;


pub struct Router<T> {
    routes: HashMap<Uri, Responder<T>>,
    error_handler: Responder<T>,
}


impl<T> Router<T> {

    /// Creates a new `Router` from a map between paths and request handles.
    pub fn from(routes: HashMap<Uri, Responder<T>>, error_handler: Responder<T>) -> Self {
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

    /// Gets the function that handles the request, for given path.
    ///
    /// The request handler
    /// Paths are expected to start with the prefix '/'.
    fn get_handler(&self, path: &Uri) -> Option<Responder<T>> {
        let mut handlers = Vec::new();

        if path == "/" {
            if let Some(root_handler) = self.routes.get(path) {
                handlers.push(*root_handler);
            }
        } else {
            for (p, handler) in self.routes.iter() {
                if path.inner.starts_with(&p.inner) & (p != "/") {
                    handlers.push(*handler);
                } 
            }
        }

        match handlers.len() {
            1 => Some(*handlers.get(0).expect("Guaranteed to have a handle")),
            0 => None,
            _ => panic!("There are more than one paths matching the pattern")
        }
    }

    /// Handle request in case of error.
    pub fn handle_error(&self, request: &Request<T>) -> Result<Response<T>> {
        (self.error_handler)(request)
    }
}

