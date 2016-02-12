extern crate hyper;

use hyper::server::{Server, Request, Response};
use std::str::FromStr;

const DEFAULT_PORT : u16 = 7000;
const INDEX_HTML : &'static str = include_str!("index.html");

fn main() {
	let port : u16 = std::env::var("PORT").map(parse_port_str).unwrap_or(DEFAULT_PORT);

	println!("starting server on 0.0.0.0:{}", port);
	Server::http(("0.0.0.0", port)).unwrap().handle(index).unwrap();
}

fn parse_port_str(port: String) -> u16 {
	u16::from_str(&port).expect("Couldn't convert PORT to a number")
}

fn index(_: Request, res: Response) {
	res.send(&INDEX_HTML.as_bytes()).expect("Couldn't write INDEX");
}

