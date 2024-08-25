use crate::http::{request::HttpRequest, response::HttpResponse};
use crate::app::resource_map::ResourceMap;


pub type RouteHandler = fn(HttpRequest, &ResourceMap) -> HttpResponse;