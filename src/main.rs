#[macro_use]

// TODO: make it a module
include!("argparser.rs");

use hyper::{Method, StatusCode};
extern crate hyper;
extern crate futures;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

use std::io::Write;
use std::net::{TcpListener, SocketAddr, IpAddr, TcpStream};
use std::io::Read;
use std::thread;
use futures::Stream;

const PHRASE: &'static str = "Hello, World!";

struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Try POSTing data to /echo");
            },
            (&Method::Post, "/echo") => {
                let mut b = req.body();
                let x : Vec<_> = b.wait().collect();
                println!("The origin is: {:#?}", x);


            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        futures::future::ok(response)
    }

}

fn main() {
    let x = parse_args();
    println!("Running on {}:{} in dir {}", x.address, x.port, x.root.display());
    let addr = SocketAddr::new(IpAddr::V4(x.address), x.port);
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();

}



