pub mod http;
pub mod server;

use http::error::{Error, Result};
use http::header::HeaderName;
use http::method::Method;
use http::request::Request;
use http::response::Response;
use http::uri::InvalidUri;
use server::HttpServer;

use std::collections::HashSet;
use std::io::Write;

use bytes::Bytes;
use flate2::{write::GzEncoder, Compression};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args.last().unwrap().clone();
    let dir2 = dir.clone();

    let _server = HttpServer::build()
        .workers(4)
        .route("/", Method::GET, Box::new(route_home))
        .route("/echo", Method::GET, Box::new(route_echo))
        .route("/user-agent", Method::GET, Box::new(route_user_agent))
        .route(
            "/files",
            Method::GET,
            Box::new(move |request: &Request<Bytes>| {
                let filename = request.uri().inner.chars().skip(7).collect::<String>();
                let file_content = std::fs::read_to_string(format!("{}/{filename}", &dir.clone()))
                    .map_err(|_| Error::from(InvalidUri))?;
                Response::builder()
                    .with_status(200)
                    .with_header(b"Content-Type", b"application/octet-stream")
                    .with_body(Bytes::from(file_content))
            }),
        )
        .route(
            "/files",
            Method::POST,
            Box::new(move |req: &Request<Bytes>| {
                let filename = req.uri().inner.chars().skip(7).collect::<String>();
                let filepath = format!("{}/{filename}", &dir2);
                let mut file =
                    std::fs::File::create(filepath).map_err(|_| Error::from(InvalidUri))?;
                let _ = file.write_all(req.body());
                Response::builder()
                    .with_status(201)
                    .with_header(b"Content-Type", b"text/plain")
                    .with_body(Bytes::from("Created"))
            }),
        )
        .route_err(Box::new(route_error))
        .bind("localhost:4221")
        .expect("Failed to bind to address")
        .run();
}

/// Routes request to home. No-op for now.
fn route_home(_: &Request<Bytes>) -> Result<Response<Bytes>> {
    Response::builder()
        .with_status(200)
        .with_body(Bytes::from(""))
}

/// Routes request to echo endpoint. Returns whatever its passed.
fn route_echo(request: &Request<Bytes>) -> Result<Response<Bytes>> {
    let body = request.uri().inner.chars().skip(6).collect::<String>();

    let mut req = Response::builder()
        .with_status(200)
        .with_header("Content-Type", b"text/plain");
    if let Some(encs) = request.get_header("Accept-Encoding") {
        let encs =
            String::from_utf8(encs.as_bytes().to_vec()).expect("Failed to convert to string");
        let encs = encs
            .split(",")
            .map(str::trim)
            .filter(|x| *x == "gzip")
            .collect::<HashSet<_>>();
        if !encs.is_empty() {
            req = req.with_header("Content-Encoding", "gzip");
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(body.as_bytes())
                .expect("Failed to write to encoder");
            let compressed_body = encoder.finish().expect("Failed to compress body");
            return req.with_body(Bytes::from(compressed_body));
        }
    }
    req.with_body(Bytes::from(body))
}

/// Routes request to user-agent endpoint. Returns the User-Agent
/// in the body.
fn route_user_agent(request: &Request<Bytes>) -> Result<Response<Bytes>> {
    let body = request
        .get_header(HeaderName::USER_AGENT)
        .expect("User-Agent header not found.");
    Response::builder()
        .with_status(200)
        .with_header("Content-Type", b"text/plain")
        .with_body(body.as_bytes().clone())
}

/// Routes request to 404.
fn route_error(_: &Request<Bytes>) -> Result<Response<Bytes>> {
    Response::builder()
        .with_status(404)
        .with_header(b"Content-Type", b"text/plain")
        .with_body(Bytes::from("Error!"))
}
