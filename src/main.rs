pub mod method;
pub mod request;
pub mod response;
pub mod server;
pub mod error;
pub mod uri;
pub mod version;
pub mod status;

use request::*;
use server::*;
use response::*;

use std::io;
use std::io::BufReader;
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
    
    println!("Connection successful!");

    let status = "HTTP/1.1 200 OK";
    let content_file = std::fs::File::open("response.html")
        .expect("Failed to open response file");
    let content = io::read_to_string(content_file)
        .expect("Faile to read response from file.");
    let length = content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");
    write!(stream, "{response}")
        .expect("Failed to write response");

}




