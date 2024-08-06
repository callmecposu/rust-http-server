use std::{ any::Any, collections::HashMap, net::TcpListener };

// http request struct
pub struct HttpRequest {}

// http response struct
pub struct HttpResponse {}

// resource hashmap type
type ResourceMap = HashMap<String, Box<dyn Any>>;

// route handler function type
type RouteHandler = fn(HttpRequest, &ResourceMap) -> HttpResponse;

#[derive(PartialEq, Eq, Hash, Debug)]
// http methods enum
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
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
        App { routes: HashMap::new(), resources: HashMap::new() }
    }

    // is used to add a route handler to the app
    pub fn route(&mut self, definition: RouteDefinition, handler: RouteHandler) -> &mut Self {
        self.routes.insert(definition, handler);
        println!("Added a new route!\n\troutes: {:#?}", self.routes);
        return self;
    }

    // is used to add a shared resource to the app
    pub fn resource<T: 'static>(&mut self, name: String, value: T) -> &mut Self {
        self.resources.insert(name, Box::new(value));
        println!("Added a new resource!\n\tresources: {:#?}", self.resources);
        return self;
    }

    // is used to call a route handler
    pub fn call(&mut self, definition: RouteDefinition, req: HttpRequest) -> &mut Self {
        let func = self.routes.get(&definition).unwrap();
        func(req, &self.resources);
        return self;
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
        .call(
            RouteDefinition { method: HttpMethod::GET, route: "/post".to_string() },
            HttpRequest {}
        );
}

fn my_handler(req: HttpRequest, resources: &ResourceMap) -> HttpResponse {
    println!("Hello from GET /post!");
    println!("Trying to access a resource from GET /post...");
    let res = resources.get("data").unwrap().downcast_ref::<HashMap<&str, i32>>().unwrap();
    println!("Got the resource in GET /post: {:#?}", res);
    HttpResponse {}
}
