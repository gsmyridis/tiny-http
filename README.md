# `tiny-http`

## Introduction

`tiny-http` is a tiny library for creating HTTP servers easily.
The library is incomplete, and its goal is educational.
Specifically, I believe it is a good, but not perfect, example of how to design a 
library, in a Rust idiomatic way, and learn concepts around HTTP servers.
Its tiny size modularity make the library easy to read, understand and extend.

To demonstrate how simple it is to make the library more complete and usable, here is an example.
At its current iteration, the library does not support all valid headers, but it takes virtually no time to add one.
Simply, got to `tiny-http/http/header/name.rs` and add the name; the designated macro will do the rest.
The same holds for available status codes.
Additional ideas can be found in the `Improvement` section below.
Feel free to contribute!

## HTTP `Request`

An HTTP request has the following form:
```
<Method> <Request-URI> <HTTP-Version>\r\n
<Header-Name1>: <Header-Value1>\r\n
<Header-Name2>: <Header-Value2>\r\n
...
<Header-NameN>: <Header-ValueN>\r\n
\r\n
<Optional-Body>
```

The `Request<T>` struct represents an HTTP request, parameterized over the type of the request body (`T`).

To construct a `Request`, use the builder pattern. 
Start by calling the `Request::builder()` method, which provides a default configuration. 
You can customize the HTTP method, uri, version, headers, and body using the corresponding `with_*` methods. 
These methods accept arguments that implement `Into` for the respective types, offering flexibility. 
For example, `with_method` works with both `Method::GET` and `"GET"`.

The request is finalized when you specify the body using the `with_body` method.
This method returns a `Result<Request<T>, Error>`, allowing for error handling during construction.
The `"Content-Length"` is filled automatically when determining the body.

Here is an example:

```rust
use tiny_http::http::Request;

let request = Request::builder()
                  .with_method("GET")
                  .with_uri("/files/btc")
                  .with_header("Accept-Encoding", b"gzip")
                  .with_body("")
                  .expect("Failed to build request")
```

`Request` has also a method `from_stream` which takes a `TcpStream` and returns a `Result<Request<Bytes>, Error>`.

## HTTP `Response`

An HTTP response has the following form:
```
<HTTP-Version> <Status-Code> <Reason-Phrase>\r\n
<Header-Name1>: <Header-Value1>\r\n
<Header-Name2>: <Header-Value2>\r\n
...
<Header-NameN>: <Header-ValueN>\r\n
\r\n
<Optional-Body>
```

`Response<T>` represents an HTTP response, parameterized by the type of its body (`T`). 
It provides an API similar to `Request<T>`, leveraging the builder pattern for flexible and ergonomic construction.
Here is an example:

```rust
use tiny_http::http::Response;

let response = Response::builder()
                    .with_version("HTTP/1.1")
                    .with_status(200)
                    .with_header("Content-Type", "text/plain")
                    .with_body("Here is your response.".to_string())
                    .expect("Failed to construct Header");
```

## `HttpServer`

`HttpServer<T>` is a struct representing the HTTP server over a TCP connection. 
It is parametrised over the type of the request's and response's body (`T`). 
An `HttpServer` is constructed with a builder. 

The server can serve multiple requests concurrently by specifying the number of worker threads in the thread pool.
Each worker thread has an atomic reference to the server's `Router<T>`, which is used to handle each request with the appropriate `Handler<T>`.
A handler is a function that takes a reference to a request and returns a response.
More specifically,

```rust
type Handler<T> = Box<Fn(&Request<T>) -> Result<Response<T>, Error> + Send + Sync + 'static>
```

This includes functions with the correct signature, as well as closures.
To specify a route, use the `route` method, and give a URI, a method, and a `Handler`.


```rust
use bytes::Bytes;
use tiny_http::http::Method;
use tiny_http::server::HttpServer;

let server = HttpServer::builder()
                    .worker(4)
                    .route(
                       "/", 
                       Method::GET, 
                       Box::new(|_|: &Request<Bytes>| 
                            Response::builder()
                            .with_body(Bytes::from(""))
                            .expect("Failed to build response.")
                    ))
                    .bind("http://localhost:4242")
                    .expect("Failed to construct Server")
                    .run()
```

Once the `bind` method is called, the server attempts to bind to the specified address, returning a `Result<HttpServer<T>, Error>`.
To run the server, simply call `run()`.

## Example Server

You can run the example server included in the `examples` directory of the library. Just run:
```
cargo run --example example_server
```

## Improvements

As mentioned in the beginning, the library is far from complete, and there are aspects that can be improved.
Here are a few ideas:

- [ ] Improve API.
- [ ] Add more header names.
- [ ] Add more status codes.
- [ ] Make header names case-insensitive.
- [ ] Make routing method work without Box.
- [ ] Make routing method work with any type that implements the appropriate `Into`.
- [ ] Work on error handling.
- [ ] Add tests.
- [ ] Improve `Router`.
- [ ] Add graceful shutdown.
- [ ] Add compression in the `HttpServer`, not the `Handler`.
