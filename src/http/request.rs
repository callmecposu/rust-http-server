use std::{ collections::HashMap, io::{ BufRead, BufReader }, net::TcpStream };

use crate::http::{ method::HttpMethod, path::HttpPath, body::HttpBody };

use super::headers::parse_headers;

// http request struct
#[derive(Default, Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: HttpPath,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    pub body: Option<HttpBody>,
}

impl HttpRequest {
    pub fn new() -> Self {
        HttpRequest { ..Default::default() }
    }

    pub fn parse(&mut self, mut stream: &TcpStream) {
        // create a bufreader
        let mut buf_reader = BufReader::new(&mut stream);
        // read the first line of the request
        let mut line = String::new();
        buf_reader.read_line(&mut line);
        let parts = line
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        // parse the request method
        self.method = match parts[0].as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::UNDEFINED,
        };
        // divide the path into path and parameters
        let path_parts = parts[1]
            .split("?")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        // parse the path
        self.path = HttpPath::from(&path_parts[0]);
        // parse the parameters
        if path_parts.len() == 2 && path_parts[1].len() != 0 {
            let raw_params = path_parts[1]
                .split("&")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            for raw_p in raw_params {
                let kv = raw_p
                    .split("=")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                self.params.insert(kv[0].clone(), kv[1].clone());
            }
        }
        // parse the headers
        parse_headers(&mut self.headers, &mut buf_reader);
        // if there is a Content-Type header, parse the body
        if let Some(cont_type) = self.headers.get(&"Content-Type".to_string()) {
            // get the content length
            let cont_len: i32 = self.headers
                .get(&"Content-Length".to_string())
                .unwrap()
                .parse::<i32>()
                .unwrap();
            // parse the body
            let mut body = HttpBody::new();
            body.parse(cont_len, &mut buf_reader);
            self.body = Some(body);
        }
        println!("Parsed the request: {:#?}", self);
    }
}
