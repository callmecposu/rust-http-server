use crate::http::{method::HttpMethod, path::HttpPath};

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct RouteDefinition {
    pub method: HttpMethod,
    pub path: HttpPath
}