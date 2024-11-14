pub mod response;
pub mod server;
pub mod error;
pub mod uri;
pub mod version;
pub mod status;
pub mod header;
pub mod body;
pub mod method;
pub mod request;

use request::Request;
use server::HttpServer;
use response::Response;
use error::Result;
use header::name::HeaderName;

use std::io::prelude::*;
use std::net::TcpStream;



fn main() {

    let server = HttpServer::bind("localhost:4221")
        .expect("Failed to bind to address");

    for stream in server.listener.incoming() {
        let stream = stream.expect("Failed to get connection");
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {

    let request = Request::from_stream(&mut stream).unwrap();
    println!("{:?}", request.method());
    println!("{:?}", request.version());
    println!("{:?}", request.uri());
    println!("{:?}", request.body());

    let response = if request.uri() == "/" {
        route_home(request)
    } else if request.uri().inner.starts_with("/echo/") {
        route_echo(request)
    } else if request.uri().inner.starts_with("/user-agent") {
        route_user_agent(request)
    } else {
        route_error(request)
    };

    let response = response.expect("Failed to get response");
    println!("");
    println!("{:?}", response.version());
    println!("{:?}", response.status());
    println!("{:?}", response.headers());
    println!("{:?}", response.body());

    write!(stream, "{response}").expect("Failed to write response");
    println!("{response}");

}


/// Routes request to home. No-op for now.
fn route_home<T>(_: Request<T>) -> Result<Response<String>> { 
    Response::builder()
        .with_status(200)
        .with_body("".into())
}

/// Routes request to echo endpoint. Returns whatever its passed.
fn route_echo<T>(request: Request<T>) -> Result<Response<String>> {
    let body = request.uri().inner.chars().skip(6).collect::<String>();
    Response::builder()
        .with_status(200)
        .with_header(b"Content-Type", b"text/plain")
        .with_body(body)
}

/// Routes request to user-agent endpoint. Returns the User-Agent
/// in the body.
fn route_user_agent<T>(request: Request<T>) -> Result<Response<String>> {
    let header_name = HeaderName::from_bytes("User-Agent".as_bytes())
        .expect("Could not construct User-Agent HeaderName");
    let body  = request.get_header(&header_name)
        .expect("User-Agent header not found.")
        .to_string();
    Response::builder()
        .with_status(200)
        .with_header(b"Content-Type", b"text/plain")
        .with_body(body)
}

/// Routes request to 404.
fn route_error<T>(_: Request<T>) -> Result<Response<String>> {
    Response::builder()
        .with_status(404)
        .with_header(b"Content-Type", b"text/plain")
        .with_body("Error!".to_string())
}


