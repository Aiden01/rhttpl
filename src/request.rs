use super::errors::*;
use super::request_url::RequestUrl;
use super::response::Response;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub enum Method {
    GET,
    PUT,
    POST,
    PATCH,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::PUT => write!(f, "PUT"),
            Method::POST => write!(f, "POST"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
}

pub struct Request<'a> {
    body: Option<&'a str>,
    headers: HashMap<&'a str, &'a str>,
    method: Method,
    req_url: RequestUrl,
}

impl<'a> Request<'a> {
    pub fn new(method: Method, url: &'a str) -> Request<'a> {
        Request {
            body: None,
            headers: HashMap::new(),
            method,
            req_url: RequestUrl::new(url),
        }
    }

    /**
     * Sets the body of the request
     */
    pub fn set_body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    /**
     * Adds a new header to the request
     */
    pub fn with_header(mut self, key: &'a str, value: &'a str) -> Self {
        self.headers.insert(key, value);
        self
    }

    /**
     * Sends the request
     */
    pub fn send(&self) -> Result<Response, Error> {
        match TcpStream::connect(self.req_url.addr()) {
            Err(_) => Err(Error::ConnectionFailed),
            Ok(mut stream) => {
                // Format the request
                let socket_header = &format!(
                    "{} {} HTTP/1.0\r\nHost: {}\r{}\n\n",
                    self.method.to_string(),
                    self.req_url.path,
                    self.req_url.host,
                    self.format_headers()
                );
                println!("Socket header format: \n {}", socket_header);
                // write to the stream
                stream.write(socket_header.as_bytes());

                // reads the response
                let mut buf = String::new();
                stream.read_to_string(&mut buf).unwrap();
                let mut headers = [httparse::EMPTY_HEADER; 16];
                let mut r = httparse::Response::new(&mut headers);

                r.parse(&buf.as_bytes());

                Ok(Response::new(r, buf.clone()))
            }
        }
    }

    fn format_headers(&self) -> String {
        let mut header = String::new();
        for (k, v) in self.headers.iter() {
            header.push_str(&format!("\n{}: {}", k, v))
        }
        header
    }
}
