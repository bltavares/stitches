extern crate hyper;
extern crate multipart;
extern crate uuid;
extern crate mustache;

use self::hyper::header;
use self::hyper::server::{Request, Response, Handler};
use self::hyper::status::StatusCode;
use self::hyper::uri::RequestUri;
use self::multipart::server::Multipart;
use self::uuid::Uuid;
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};

const INDEX_HTML: &'static str = include_str!("index.html");
const PATCH_HTML: &'static str = include_str!("patch.html");

lazy_static! {
    static ref PATCH_TEMPLATE: mustache::Template = {
        mustache::compile_str(PATCH_HTML)
    };
}

#[derive(Debug)]
pub struct Storage {
    patches: Arc<Mutex<HashMap<Uuid, String>>>,
}

fn redirect(mut res: Response, location: String) {
    *res.status_mut() = StatusCode::TemporaryRedirect;
    res.headers_mut().set(header::Location(location));
}

impl Storage {
    pub fn new() -> Storage {
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

            let uuid = self.insert(patch_content);

            redirect(res, format!("/patches/{}", uuid));
        } else {
            redirect(res, "/".to_owned());
        }
    }

    fn view_patch(&self, _: Request, res: Response, id: &str) {
        let uuid = Uuid::parse_str(&id).expect("Could not parse id");
        let patch = match self.get(&uuid) {
            Some(p) => p,
            None => {
                redirect(res, "/".to_owned());
                return;
            }
        };

        let mut data = HashMap::new();
        data.insert("id", id);
        data.insert("patch", &patch);

        let mut buffer = Vec::new();
        PATCH_TEMPLATE.render(&mut buffer, &data).expect("Could not parse template");

        res.send(&buffer).expect("Could not write the patch view");
    }

    fn get(&self, id: &Uuid) -> Option<String> {
        let patches_ref = self.patches.clone();
        let locked_patches = patches_ref.lock().expect("Could not aquire lock");
        locked_patches.get(id).cloned()
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
            } else if url.starts_with("/patches/") {
                self.view_patch(req, res, &url[9..]);
            } else {
                self.index(req, res);
            }
        } else {
            self.index(req, res);
        }
    }
}
