extern crate hyper;
extern crate multipart;

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri;
use multipart::server::Multipart;
use std::str::FromStr;

const DEFAULT_PORT: u16 = 7000;
const INDEX_HTML: &'static str = include_str!("index.html");

fn main() {
    let port: u16 = std::env::var("PORT").map(parse_port_str).unwrap_or(DEFAULT_PORT);

    println!("starting server on 0.0.0.0:{}", port);
    Server::http(("0.0.0.0", port)).unwrap().handle(router).unwrap();
}

fn parse_port_str(port: String) -> u16 {
    u16::from_str(&port).expect("Couldn't convert PORT to a number")
}

fn router(req: Request, res: Response) {
    if let RequestUri::AbsolutePath(url) = req.uri.clone() {
        if url == "/patches/new" {
            new_patch(req, res);
        } else {
            index(req, res);
        }
    } else {
        index(req, res);
    }

}

fn index(_: Request, res: Response) {
    res.send(&INDEX_HTML.as_bytes()).expect("Couldn't write INDEX");
}

fn new_patch(req: Request, res: Response) {
    if let Ok(mut multi_req) = Multipart::from_request(req) {
        multi_req.foreach_entry(|x| println!("Entry: {}", x.name))
            .expect("Could not read entries on new patch");
    }
}
