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

use server::HttpServer;
use request::Request;
use response::Response;
use error::Result;
use header::name::HeaderName;


fn main() {

    let _server = HttpServer::build()
        .verbose(true)
        .workers(4)
        .route("/", route_home)
        .route("/echo", route_echo)
        .route("/user-agent", route_user_agent)
        .route_err(route_error)
        .bind("localhost:4221")
        .expect("Failed to bind to address")
        .run();
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

