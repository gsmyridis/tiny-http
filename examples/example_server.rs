use tiny_http::http::{Error, Result, Method, Request, Response, InvalidUri};
use tiny_http::server::HttpServer;

use bytes::Bytes;
use std::fs::read_to_string;

fn main() {

    HttpServer::build()
        .workers(4)
        .route("/", Method::GET, Box::new(route_home))
        .route(
            "/files",
            Method::GET,
            Box::new(move |request: &Request<Bytes>| {
                let filename = request.uri().inner.chars().skip(7).collect::<String>();
                let file_content = read_to_string(format!("examples/files/{filename}"))
                    .map_err(|_| Error::from(InvalidUri))?;
                Response::builder()
                    .with_status(200)
                    .with_header("Content-Type", b"application/octet-stream")
                    .with_body(Bytes::from(file_content))
            }),
        )
        .route_err(Box::new(route_error))
        .bind("localhost:4221")
        .expect("Failed to bind to address")
        .run();
}

fn route_home(_: &Request<Bytes>) -> Result<Response<Bytes>> {
    Response::builder()
        .with_status(200)
        .with_body(Bytes::from(""))
}

fn route_error(_: &Request<Bytes>) -> Result<Response<Bytes>> {
    Response::builder()
        .with_status(404)
        .with_header("Content-Type", b"text/plain")
        .with_body(Bytes::from("Error!"))
}
