use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
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
