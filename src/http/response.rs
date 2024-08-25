use std::collections::HashMap;

use serde::Serialize;

use super::body::HttpBody;

// http response struct
#[derive(Default, Debug)]
pub struct HttpResponse {
    pub status_code: i32,
    pub status_message: String,
    pub headers: HashMap<String, String>,
    pub body: Option<HttpBody>,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {
            ..Default::default()
        }
    }

    pub fn with_status_code(&mut self, code: i32) -> &mut Self {
        self.status_code = code;
        return self;
    }

    pub fn with_status_msg(&mut self, msg: &str) -> &mut Self {
        self.status_message = msg.to_string();
        return self;
    }

    pub fn with_header(&mut self, k: &str, v: &str) -> &mut Self {
        self.headers.insert(k.to_string(), v.to_string()).unwrap();
        return self;
    }

    pub fn with_json<T: Serialize>(&mut self, payload: T) -> &mut Self {
        let mut body = HttpBody::new();
        let json_string = serde_json::to_string(&payload).unwrap();
        body.raw = json_string;
        self.body = Some(body);
        return self;
    }

    pub fn prepare(&self) -> String {
        // prepare the response to be sent
        let mut raw = String::new();
        // write the first line
        raw.push_str(format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_message).as_str());
        // write the headers
        for (k, v) in self.headers.iter() {
            raw.push_str(format!("{}:{}\r\n", *k, *v).as_str());
        }
        // if response has a body, write its length to the headers and the body itself
        if let Some(body) = &self.body {
            raw.push_str("Content-Type: application/json");
            raw.push_str(format!("Content-Length: {}\r\n\r\n", body.raw.len()).as_str());
            raw.push_str(&body.raw);
        } else {
            raw.push_str("\r\n");
        }
        println!("Prepared response: \n{}", &raw);
        return raw;
    }
}
