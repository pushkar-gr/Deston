mod config;
mod load_balancer;
mod server;
use crate::load_balancer::load_balancer::LoadBalancer;
use config::config::Config;
use load_balancer::layer4::Layer4;
use std::path::Path;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let config = Config::new(Path::new("config.toml"));
    let lb = Layer4::new(Arc::new(Mutex::new(config)));
    let _ = lb.start().await;
}
