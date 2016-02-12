extern crate hyper;

use hyper::server::{Server, Handler};
use std::str::FromStr;

mod web;

const DEFAULT_PORT: u16 = 7000;

fn main() {
    let port: u16 = std::env::var("PORT").map(parse_port_str).unwrap_or(DEFAULT_PORT);

    println!("starting server on 0.0.0.0:{}", port);
    Server::http(("0.0.0.0", port)).unwrap().handle(web::Storage::new()).unwrap();
}

fn parse_port_str(port: String) -> u16 {
    u16::from_str(&port).expect("Couldn't convert PORT to a number")
}
