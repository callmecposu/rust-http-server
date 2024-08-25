use std::collections::HashMap;
use std::io::Write;
use std::net::{ TcpListener, TcpStream };

use crate::http::path::HttpPath;
use crate::app::{
    resource_map::ResourceMap,
    route_handler::RouteHandler,
    route_definition::RouteDefinition,
};
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;

// the application struct
pub struct App {
    // routes hashmap
    routes: HashMap<RouteDefinition, RouteHandler>,
    // shared resources hashmap
    resources: ResourceMap,
}

impl App {
    pub fn new() -> Self {
        App { routes: HashMap::new(), resources: ResourceMap::new() }
    }

    // is used to add a route handler to the app
    pub fn route(&mut self, definition: RouteDefinition, handler: RouteHandler) -> &mut Self {
        self.routes.insert(definition, handler);
        println!("Added a new route!\n\troutes: {:#?}", self.routes);
        return self;
    }

    // is used to add a shared resource to the app
    pub fn resource<T: 'static>(&mut self, name: String, value: T) -> &mut Self {
        self.resources.add_resource(name, value);
        return self;
    }

    // is used to call a route handler
    pub fn call(&mut self, definition: RouteDefinition, req: HttpRequest) -> &mut Self {
        let handler = self.routes.get(&definition).unwrap();
        handler(req, &self.resources);
        return self;
    }

    pub fn run(&mut self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(&stream);
        }
    }

    fn handle_connection(&mut self, mut stream: &TcpStream) {
        println!("Accepted a Connection!");
        // parse the request
        let mut req = HttpRequest::new();
        req.parse(&stream);
        // search for a matching handler
        let mut handler: Option<RouteHandler> = None;
        for (k, v) in self.routes.iter() {
            if k.method == req.method {
                let m = req.path.does_match(&k.path);
                if m {
                    handler = Some(*v);
                    break;
                }
            }
        }
        // prepare a response
        let resp = match handler {
            Some(h) => {
                h(req, &self.resources)
            },
            None => {
                let mut resp = HttpResponse::new();
                resp.with_status_code(404).with_status_msg("NOT FOUND");
                resp
            }
        };
        stream.write_all(resp.prepare().as_bytes()).unwrap();
    }
}
