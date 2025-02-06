mod config;
mod load_balancer;
mod server;
use crate::load_balancer::load_balancer::LoadBalancer;
use config::config::{Algorithm, Config};
use load_balancer::layer4::Layer4;
use load_balancer::layer7::Layer7;
use server::server::Server;

use hyper::Uri;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let server1 = Server::new("http://127.0.0.1:3000".parse::<Uri>().unwrap());
    let server2 = Server::new("http://127.0.0.1:3001".parse::<Uri>().unwrap());
    let config = Config::new(
        Arc::new(vec![
            Arc::new(Mutex::new(server1)),
            Arc::new(Mutex::new(server2)),
        ]),
        Algorithm::RoundRobin,
    );
    //let lb = Layer7::new(Arc::new(Mutex::new(config)));
    let lb = Layer4::new(Arc::new(Mutex::new(config)));
    let _ = lb.start().await;
}
