use std::io;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};


use std::str::FromStr;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
enum RequestMethod {
    Get,
    Post,
}

#[derive(Debug)]
struct RequestMethodError(String);

impl Display for RequestMethodError {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Request method can be either 'GET', or 'POST'. Instead got: '{}'.", self.0)
    }

}

impl Error for RequestMethodError {}


impl FromStr for RequestMethod {
    type Err = RequestMethodError;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "GET" => Ok(RequestMethod::Get),
            "POST" => Ok(RequestMethod::Post),
            _ => Err(RequestMethodError(input.to_string())),
        }
    }

}



#[derive(Debug)]
struct HttpRequest {
    method: RequestMethod,
    uri: String,
    version: String,
}

impl HttpRequest {
    
    fn from_reader<T: Read + BufRead>(reader: T) -> Self {
        let mut request_lines = reader.lines();
        let request_header = request_lines.next()
            .expect("Failed to read next line from request reader.")
            .expect("There is no header line.");

        let mut header_split = request_header.split(' ');

        let method = match header_split.next() {
            Some(method_str) => {
                RequestMethod::from_str(method_str).expect("Failed to parses request method")
            },
            _ => panic!("Did not receive appropriate request: request kind missing")
        };

        let uri = match header_split.next() {
            Some(uri) => uri.to_string(),
            _ => panic!("Did not receive appropriate request: URI missing")
        };

        let version = match header_split.next() {
            Some(version) if version == "HTTP/1.1" => version.to_string(),
            Some(_) => panic!("Did not receive appropriate request: HTTP version wrong or missing."),
            None => panic!("Did not receive appropriate request: HTTP version wrong or missing.")
        };
        
        HttpRequest{ method, uri, version }

    }
    
}


struct HttpServer {
    listener: TcpListener,
}


impl HttpServer {
    
    fn bind(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }
}


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
    let mut bufreader = BufReader::new(&mut stream);
    
    let request = HttpRequest::from_reader(&mut bufreader);
    println!("{:?}", request);
    println!("Connection successful!");

//    let status = "HTTP/1.1 200 OK";
//    let content_file = std::fs::File::open("response.html")
//        .expect("Failed to open response file");
//    let content = io::read_to_string(content_file)
//        .expect("Faile to read response from file.");
//    let length = content.len();
//    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");
//    write!(stream, "{response}")
//        .expect("Failed to write response");

}




