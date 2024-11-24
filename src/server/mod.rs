pub mod build;
pub mod pool;
pub mod router;
pub mod worker;

use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::Arc;

use bytes::Bytes;

use build::Builder;
use pool::ThreadPool;
use router::Router;

use crate::http::error::Result;
use crate::http::request::Request;
use crate::http::response::Response;

type RequestHandler<T> = Box<dyn Fn(&Request<T>) -> Result<Response<T>> + Send + Sync + 'static>;

pub struct HttpServer<T> {
    listener: TcpListener,
    pool: ThreadPool,
    router: Arc<Router<T>>,
}

impl<T> HttpServer<T> {
    /// Returns an HTTP server builder.
    pub fn build() -> Builder<T> {
        Builder::default()
    }
}

impl HttpServer<Bytes> {
    /// Starts the HTTP server.
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.expect("Failed to get connection");
            let router = Arc::clone(&self.router);
            self.pool.execute(move || {
                let request = Request::from_stream(&mut stream).unwrap();
                let response = match router.handle_request(&request) {
                    Ok(response) => response,
                    _ => router
                        .handle_error(&request)
                        .expect("Guaranteed to get response"),
                };

                write!(
                    stream,
                    "{} {} {}\r\n",
                    response.version(),
                    response.status().code(),
                    response.status().msg().expect("Guaranteed.")
                )
                .expect("Failed to write response");

                for (name, val) in response.headers() {
                    write!(stream, "{}: {}\r\n", name, val).expect("Failed to write headers");
                }
                write!(stream, "\r\n").expect("Failed to write CRLF");
                stream
                    .write_all(response.body())
                    .expect("Failed to write body");
            });
        }
    }
}
