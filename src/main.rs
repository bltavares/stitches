extern crate hyper;
extern crate multipart;
extern crate uuid;

use hyper::server::{Server, Request, Response, Handler};
use hyper::uri::RequestUri;
use multipart::server::Multipart;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

const DEFAULT_PORT: u16 = 7000;
const INDEX_HTML: &'static str = include_str!("index.html");

#[derive(Debug)]
struct Storage {
    patches: Arc<Mutex<HashMap<Uuid, String>>>,
}

impl Storage {
    fn new() -> Storage {
        Storage { patches: Arc::new(Mutex::new(HashMap::new())) }
    }

    fn index(&self, _: Request, res: Response) {
        res.send(&INDEX_HTML.as_bytes()).expect("Couldn't write INDEX");
    }

    fn new_patch(&self, req: Request, res: Response) {
        if let Ok(mut multi_req) = Multipart::from_request(req) {
            let mut patch_content = String::new();
            multi_req.foreach_entry(|mut entry| {
                         entry.data
                              .as_file()
                              .unwrap()
                              .read_to_string(&mut patch_content)
                              .expect("Could not read the conten of the patch");
                     })
                     .expect("Could not read entries on new patch");

            self.insert(patch_content);
        }
    }

    fn insert(&self, patch: String) -> String {
        let uuid = Uuid::new_v4();
        let patches_ref = self.patches.clone();
        let mut locked_patches = patches_ref.lock().expect("Could not aquire lock");
        locked_patches.insert(uuid, patch);
        uuid.to_simple_string()
    }
}

impl Handler for Storage {
    fn handle(&self, req: Request, res: Response) {
        if let RequestUri::AbsolutePath(url) = req.uri.clone() {
            if url == "/patches/new" {
                self.new_patch(req, res);
            } else {
                self.index(req, res);
            }
        } else {
            self.index(req, res);
        }
    }
}

fn main() {
    let port: u16 = std::env::var("PORT").map(parse_port_str).unwrap_or(DEFAULT_PORT);

    println!("starting server on 0.0.0.0:{}", port);
    Server::http(("0.0.0.0", port)).unwrap().handle(Storage::new()).unwrap();
}

fn parse_port_str(port: String) -> u16 {
    u16::from_str(&port).expect("Couldn't convert PORT to a number")
}
