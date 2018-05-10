extern crate hyper;
extern crate futures;

use futures::Future;

struct ForestService;

impl hyper::server::Service for ForestService {
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response,Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(futures::future::ok(Self::Response::new()
                                     .with_body("test")))
    }
}

fn main() {
    let addr = std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)), 
        std::env::var("PORT").ok().and_then(|s| Some(s.parse().unwrap())).unwrap_or(7777));
    let server = hyper::server::Http::new().bind(&addr, || Ok(ForestService)).unwrap();
    server.run().unwrap();
}
