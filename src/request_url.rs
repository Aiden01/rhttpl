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

use std::net::{SocketAddr, ToSocketAddrs};
use url::Url;

pub struct RequestUrl {
    pub path: String,
    pub host: String,
    pub port: u16,
}

impl<'a> RequestUrl {
    pub fn new(url: &str) -> RequestUrl {
        match Url::parse(url) {
            Ok(domain) => RequestUrl {
                path: domain.path().to_string(),
                host: domain.host_str().unwrap().to_string(),
                port: domain.port_or_known_default().unwrap(),
            },
            Err(_) => panic!("Invalid URL."),
        }
    }

    /**
     * Returns a socket addr
     */
    pub fn addr(&self) -> SocketAddr {
        let host = &format!("{}:{}", self.host, self.port);
        println!("Host: {}", host);
        let addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
        addrs[0]
    }
}
