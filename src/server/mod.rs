pub mod pool;
pub mod worker;
pub mod build;
pub mod router;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::Arc;

use pool::ThreadPool;
use build::Builder;
use router::Router;

use crate::request::Request;
use crate::response::Response;
use crate::error::Result;


type RequestHandler<T> = fn(Request<T>) -> Result<Response<T>>;


pub struct HttpServer {
    verbose: bool,
    listener: TcpListener,
    pool: ThreadPool,
    router: Arc<Router<String>>,
}


impl HttpServer {

    /// Retuns an HTTP server builder.
    pub fn build() -> Builder {
        Builder::default()
    }

    /// Starts the HTTP server.
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.expect("Failed to get connection");
            let router = Arc::clone(&self.router);
            let verbose = self.verbose;
            self.pool.execute(move || route(stream, router, verbose));
        }
    }
}


fn route(mut stream: TcpStream, router: Arc<Router<String>>, verbose: bool) {

    let request = Request::from_stream(&mut stream).unwrap();
    if verbose {
        println!("");
        println!("REQUEST");
        println!("Method: {}", request.method());
        println!("Version: {}", request.version());
        println!("Uri: {}", request.uri());
        println!("Body: {}", request.body());
    }

    let handler = router.get_handler(request.uri())
        .unwrap_or(router.error_handler.expect("No such path and no default error handler."));
    let response = handler(request).expect("Failed to get response");

    if verbose {
        println!("");
        println!("RESPONSE");
        println!("Version: {}", response.version());
        println!("Status: {}", response.status());
        println!("{:?}", response.headers());
        println!("{}", response.body());
    }

    write!(stream, "{response}").expect("Failed to write response");

}
