use super::body::Body;
use httparse::Header;
use httparse::Response as ParsedResponse;
use std::collections::HashMap;
use std::str;

type ResponseHeader = HashMap<String, String>;

pub enum Status {
    Ok,
}

pub struct Response {
    pub status_code: u16,
    data: String,
    pub http_version: u8,
    pub headers: ResponseHeader,
}

impl Response {
    pub fn new(r: ParsedResponse, data: String) -> Response {
        Response {
            status_code: r.code.unwrap(),
            data,
            http_version: r.version.unwrap(),
            headers: get_headers(r.headers),
        }
    }

    pub fn body(&self) -> Body {
        Body {
            text: self.data.clone(),
        }
    }
}

fn get_headers(headers: &[Header]) -> ResponseHeader {
    let mut r_headers = HashMap::new();
    for h in headers {
        r_headers.insert(
            h.name.to_string(),
            str::from_utf8(h.value).unwrap().to_string(),
        );
    }
    r_headers
}
