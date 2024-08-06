use std::{
    any::Any,
    collections::HashMap,
    default,
    io::{ BufRead, BufReader, Read },
    net::{ TcpListener, TcpStream },
};

#[derive(Default, Debug)]
pub struct HttpHeader {
    method: HttpMethod,
    path: String,
    headers: HashMap<String, String>,
}

impl HttpHeader {
    pub fn new() -> Self {
        HttpHeader { ..Default::default() }
    }

    pub fn parse(&mut self, buf_reader: &mut BufReader<&mut &TcpStream>) {
        let mut lines = Vec::<String>::new();
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line);
            if line.trim().is_empty() {
                break;
            }
            lines.push(line.trim().to_string());
        }
        println!("header lines: {:#?}", lines);
        self.method = match lines[0].split(' ').collect::<Vec<&str>>()[0] {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::UNDEFINED,
        };
        self.path = lines[0]
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()[1]
            .clone();
        for l in &lines[1..] {
            self.headers.insert(
                l
                    .split(':')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()[0]
                    .trim()
                    .to_string(),
                l
                    .split(':')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()[1]
                    .trim()
                    .to_string()
            );
        }
        println!("Parsed header: {:#?}", self);
    }
}

#[derive(Default)]
pub struct HttpBody {
    raw: String,
}

impl HttpBody {
    pub fn parse(&mut self, buf_reader: &mut BufReader<&mut &TcpStream>, length: i32) {
        let mut bytes = Vec::<u8>::new();
        let mut byte_buf: [u8; 1] = [0];
        for _ in 0..length {
            buf_reader.read_exact(&mut byte_buf);
            bytes.push(byte_buf[0]);
        }
        let raw = bytes
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>();
        println!("body: {}", raw);
        self.raw = raw;
    }
}

// http request struct
#[derive(Default)]
pub struct HttpRequest {
    header: HttpHeader,
    body: Option<HttpBody>,
}

impl HttpRequest {
    pub fn new() -> Self {
        HttpRequest { ..Default::default() }
    }

    pub fn parse(&mut self, mut stream: &TcpStream) {
        // create a bufreader
        let mut buf_reader = BufReader::new(&mut stream);
        self.header.parse(&mut buf_reader);
        match self.header.headers.get(&"Content-Type".to_string()) {
            Some(v) => {
                match v.as_str() {
                    "application/json" => {
                        let mut body = HttpBody { ..Default::default() };
                        let length = self.header.headers
                            .get(&"Content-Length".to_string())
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
                        body.parse(&mut buf_reader, length);
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }
}

// http response struct
pub struct HttpResponse {}

// resource hashmap struct
// type ResourceMap = HashMap<String, Box<dyn Any>>;
pub struct ResourceMap {
    resources: HashMap<String, Box<dyn Any>>,
}

impl ResourceMap {
    pub fn new() -> Self {
        ResourceMap { resources: HashMap::<String, Box<dyn Any>>::new() }
    }

    pub fn add_resource<T: 'static>(&mut self, name: String, value: T) -> &mut Self {
        self.resources.insert(name, Box::new(value));
        return self;
    }

    pub fn get_resource<T: 'static>(&self, name: String) -> Option<&T> {
        let res = match self.resources.get(&name) {
            Some(v) => v,
            None => {
                return None;
            }
        };
        return res.downcast_ref::<T>();
    }
}

// route handler function type
type RouteHandler = fn(HttpRequest, &ResourceMap) -> HttpResponse;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
// http methods enum
pub enum HttpMethod {
    UNDEFINED,
    GET,
    POST,
    PUT,
    DELETE,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::UNDEFINED
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
// the route definition struct
pub struct RouteDefinition {
    // route method
    method: HttpMethod,
    route: String,
}

// the application struct
pub struct App {
    // tcp listener
    // listener: TcpListener,
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
        let func = self.routes.get(&definition).unwrap();
        func(req, &self.resources);
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
        let req_rdef = RouteDefinition {
            method: req.header.method,
            route: req.header.path.clone(),
        };
        let handler = self.routes.get(&req_rdef).unwrap();
        handler(req, &self.resources);
    }
}

fn main() {
    let mut app = App::new();
    app.route(RouteDefinition { method: HttpMethod::GET, route: "/post".to_string() }, my_handler)
        .resource(
            "data".to_string(),
            HashMap::<&str, i32>::from([
                ("lol", 1),
                ("kek", 2),
            ])
        )
        // .call(
        //     RouteDefinition { method: HttpMethod::GET, route: "/post".to_string() },
        //     HttpRequest {}
        // )
        .run("127.0.0.1:7878");
}

fn my_handler(req: HttpRequest, resources: &ResourceMap) -> HttpResponse {
    println!("Hello from GET /post!");
    println!("Trying to access a resource from GET /post...");
    let res = resources.get_resource::<HashMap<&str, i32>>("data".to_string()).unwrap();
    println!("Got the resource in GET /post: {:#?}", res);
    HttpResponse {}
}
