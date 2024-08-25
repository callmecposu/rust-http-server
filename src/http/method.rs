#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum HttpMethod{
    UNDEFINED,
    GET,
    PUT,
    POST,
    DELETE
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::UNDEFINED
    }
}