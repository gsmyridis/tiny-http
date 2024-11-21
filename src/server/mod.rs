pub mod pool;
pub mod worker;
pub mod build;
pub mod router;
use std::net::TcpListener; 
use std::io::prelude::*;
use std::sync::Arc;

use pool::ThreadPool;
use build::Builder;
use router::Router;

use crate::request::Request;
use crate::response::Response;
use crate::error::Result;


type RequestHandler<T> = Box<dyn Fn(&Request<T>) -> Result<Response<T>> + Send + Sync + 'static>;


pub struct HttpServer<T> {
    verbose: bool,
    listener: TcpListener,
    pool: ThreadPool,
    router: Arc<Router<T>>,
}


impl HttpServer<String> {

    /// Retuns an HTTP server builder.
    pub fn build() -> Builder<String> {
        Builder::default()
    }

    /// Starts the HTTP server.
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.expect("Failed to get connection");
            let router = Arc::clone(&self.router);
            let verbose = self.verbose;
            self.pool.execute(move || {
                let request = Request::from_stream(&mut stream).unwrap();
                if verbose {
                    println!("");
                    println!("REQUEST");
                    println!("Method: {}", request.method());
                    println!("Version: {}", request.version());
                    println!("Uri: {}", request.uri());
                    println!("Body: {}", request.body());
                }

                let response = match router.handle_request(&request) {
                    Ok(response) => response,
                    _ => router.handle_error(&request).expect("Guaranteed to get response")
                };

                if verbose {
                    println!("");
                    println!("RESPONSE");
                    println!("Version: {}", response.version());
                    println!("Status: {}", response.status());
                    println!("{:?}", response.headers());
                    println!("{}", response.body());
                }

                write!(stream, "{response}").expect("Failed to write response");
            });
        }
    }
}
