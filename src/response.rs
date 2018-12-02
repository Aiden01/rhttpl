// Copyright 2018 webd
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
