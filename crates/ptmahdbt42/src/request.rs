use crate::CRLF;
use std::{collections::HashMap, fmt::Display};

pub struct Request {
    host: String,
    port: u16,
    method: Method,
    path: String,
    headers: HashMap<String, String>,
    body: Option<Body>,
}

impl Request {
    pub fn new(
        host: String,
        port: u16,
        method: Method,
        path: String,
        headers: HashMap<String, String>,
        body: Option<Body>,
    ) -> Self {
        Request {
            host,
            port,
            method,
            path,
            headers,
            body,
        }
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::CONNECT => "CONNECT",
            Method::TRACE => "TRACE",
        };
        write!(f, "{}", method_str)
    }
}

pub enum Body {
    String(String),
    Bytes(Vec<u8>),
}

impl Body {
    fn to_bytes(&self) -> &[u8] {
        match self {
            Body::String(s) => s.as_bytes(),
            Body::Bytes(b) => b,
        }
    }
}

pub(crate) fn build_request_package(request: &Request) -> Vec<u8> {
    let mut request_vec: Vec<u8> = vec![];

    request_vec
        .extend_from_slice(format!("{} {} HTTP/1.0", request.method, request.path).as_bytes());
    request_vec.extend_from_slice(CRLF);

    if request.port == 80 {
        request_vec.extend_from_slice(format!("Host: {}", request.host).as_bytes());
    } else {
        request_vec
            .extend_from_slice(format!("Host: {}:{}", request.host, request.port).as_bytes());
    }
    request_vec.extend_from_slice(CRLF);

    for (header_name, header_value) in &request.headers {
        request_vec.extend_from_slice(format!("{}: {}", header_name, header_value).as_bytes());
        request_vec.extend_from_slice(CRLF);
    }

    if let Some(body) = &request.body {
        let body_bytes = body.to_bytes();
        request_vec.extend_from_slice(format!("Content-Length: {}", body_bytes.len()).as_bytes());
        request_vec.extend_from_slice(CRLF);
        request_vec.extend_from_slice(CRLF);
        request_vec.extend_from_slice(body_bytes);
    } else {
        request_vec.extend_from_slice(CRLF);
    }

    request_vec
}
