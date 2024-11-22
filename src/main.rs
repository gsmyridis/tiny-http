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
use method::Method;
use header::name::HeaderName;
use error::{Result, Error};
use uri::InvalidUri;

use std::io::Write;


fn main() {

    let args: Vec<String> = std::env::args().collect();
    let dir = args.last().unwrap().clone();
    let dir2 = dir.clone();


    let _server = HttpServer::build()
        .verbose(true)
        .workers(4)
        .route("/", Method::GET, Box::new(route_home))
        .route("/echo", Method::GET, Box::new(route_echo))
        .route("/user-agent", Method::GET, Box::new(route_user_agent))
        .route("/files", Method::GET, Box::new(move |request: &Request<String>| {
            let filename = request.uri().inner.chars().skip(7).collect::<String>();
            let file_content = std::fs::read_to_string(format!("{}/{filename}", &dir.clone()))
                                        .map_err(|_| Error::from(InvalidUri))?;
            Response::builder()
                .with_status(200)
                .with_header(b"Content-Type", b"application/octet-stream")
                .with_body(file_content)
        }))
        .route("/files", Method::POST, Box::new(move |req: &Request<String>| {
            let filename = req.uri().inner.chars().skip(7).collect::<String>();
            let filepath = format!("{}/{filename}", &dir2);
            println!("PATH: {filepath}");
            let mut file = std::fs::File::create(filepath).expect("Failed to create file"); // .map_err(|_| Error::from(InvalidUri))?;
            let _ = file.write_all(req.body().as_bytes());
            Response::builder()
                .with_status(201)
                .with_header(b"Content-Type", b"text/plain")
                .with_body("Created".to_string())
        }))
        .route_err(Box::new(route_error))
        .bind("localhost:4221")
        .expect("Failed to bind to address")
        .run();
}


/// Routes request to home. No-op for now.
fn route_home(_: &Request<String>) -> Result<Response<String>> { 
    Response::builder()
        .with_status(200)
        .with_body("".to_string())
}

/// Routes request to echo endpoint. Returns whatever its passed.
fn route_echo(request: &Request<String>) -> Result<Response<String>> {
    let body = request.uri().inner.chars().skip(6).collect::<String>();
    Response::builder()
        .with_status(200)
        .with_header(b"Content-Type", b"text/plain")
        .with_body(body)
}

/// Routes request to user-agent endpoint. Returns the User-Agent
/// in the body.
fn route_user_agent(request: &Request<String>) -> Result<Response<String>> {
    let header_name = HeaderName::from_bytes("User-Agent".as_bytes())
        .expect("Could not construct User-Agent HeaderName");
    let body  = request.get_header(&header_name)
        .expect("User-Agent header not found.");
    Response::builder()
        .with_status(200)
        .with_header(b"Content-Type", b"text/plain")
        .with_body(body.to_string())
}


/// Routes request to 404.
fn route_error(_: &Request<String>) -> Result<Response<String>> {
    Response::builder()
        .with_status(404)
        .with_header(b"Content-Type", b"text/plain")
        .with_body("Error!".into())
}
