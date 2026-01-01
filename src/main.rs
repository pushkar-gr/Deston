mod config;
mod load_balancer;
mod server;
use crate::load_balancer::load_balancer::LoadBalancer;
use config::config::{Config, LayerMode};
use load_balancer::layer4::Layer4;
use load_balancer::layer7::Layer7;
use std::path::Path;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let config = Config::new(Path::new("config.toml"));
    let layer_mode = config.layer_mode.clone();
    let config_arc = Arc::new(Mutex::new(config));

    match layer_mode {
        LayerMode::L4 => {
            println!("Starting Layer 4 (TCP) Load Balancer...");
            let lb = Layer4::new(config_arc);
            if let Err(e) = lb.start().await {
                eprintln!("Error starting Layer 4 load balancer: {:?}", e);
            }
        }
        LayerMode::L7 => {
            println!("Starting Layer 7 (HTTP) Load Balancer...");
            let lb = Layer7::new(config_arc);
            if let Err(e) = lb.start().await {
                eprintln!("Error starting Layer 7 load balancer: {:?}", e);
            }
        }
    }
}
