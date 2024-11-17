use std::collections::HashMap;

use crate::uri::Uri;
use super::RequestHandler;


pub struct Router<T> {
    routes: HashMap<Uri, RequestHandler<T>>,
    pub error_handler: Option<RequestHandler<T>>,
}


impl<T> Router<T> {

    /// Creates a new `Router` from a map between paths and request handles.
    pub fn from(routes: HashMap<Uri, RequestHandler<T>>, error_handler: Option<RequestHandler<T>>) -> Self {
        Router{ routes, error_handler }
    }

    /// Gets the function that handles the request, for given path.
    ///
    /// The request handler
    /// Paths are expected to start with the prefix '/'.
    pub fn get_handler(&self, path: &Uri) -> Option<RequestHandler<T>> {
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
}

