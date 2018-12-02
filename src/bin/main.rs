extern crate rhttpl;

use rhttpl::request::Method;
use rhttpl::request::Request;
use rhttpl::response::Response;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;

fn main() {
    let host = "jsonplaceholder.typicode.com:8080";
    let addrs_iter = host.to_socket_addrs().unwrap();
    let addrs: Vec<SocketAddr> = addrs_iter.collect();
    println!("{:?}", addrs[0]);
    let res: Response = Request::new(Method::GET, "http://jsonplaceholder.typicode.com/posts")
        .send()
        .unwrap();
}
