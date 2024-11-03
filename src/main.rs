pub mod method;
pub mod request;
pub mod response;
pub mod server;
pub mod error;
pub mod uri;
pub mod version;
pub mod status;

use request::Request;
use server::HttpServer;
use response::Response;

use std::io::prelude::*;
use std::net::TcpStream;


fn main() {

    let server = HttpServer::bind("127.0.0.1:7878")
        .expect("Failed to bind to address");

    for stream in server.listener.incoming() {
        let stream = stream
            .expect("Failed to get connection");

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    
    let request = Request::from_stream(&mut stream).unwrap();
    println!("{:?}", request.method());
    println!("{:?}", request.version());
    println!("{:?}", request.uri());
    println!("{:?}", request.body());

    let content = std::fs::read_to_string("response.html")
        .expect("Faile to read response from file.");
    let response = Response::builder()
        .with_status(200)
        .with_body(content)
        .expect("Failed to construct response");


    let response_line = format!(
        "{} {} {}\r\n\r\n", response.version(), response.status().code(), response.status().msg().unwrap()
    );
    write!(stream, "{response_line}")
        .expect("Failed to write response");

}




