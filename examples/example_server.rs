use tiny_http::http::{Error, Result, Method, Request, Response, InvalidUri};
use tiny_http::server::HttpServer;

use bytes::Bytes;
use std::fs::read_to_string;

fn main() {
    
    let server = HttpServer::build()
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
                    .with_header("Content-Type", b"text/html")
                    .with_body(Bytes::from(file_content))
            }),
        )
        .route_err(Box::new(route_error))
        .bind("localhost:4221")
        .expect("Failed to bind to address");

    println!("\nOpen your browser and visit http://localhost:4221");
    server.run();
}

fn route_home(_: &Request<Bytes>) -> Result<Response<Bytes>> {
    let home = read_to_string("examples/files/home.html")
        .expect("Failed to read file.");
    Response::builder()
        .with_status(200)
        .with_body(Bytes::from(home))
}

fn route_error(_: &Request<Bytes>) -> Result<Response<Bytes>> {
    let err = read_to_string("examples/files/error.html")
        .expect("Failed to read file.");
    Response::builder()
        .with_status(404)
        .with_header("Content-Type", b"text/html")
        .with_body(Bytes::from(err))
}
