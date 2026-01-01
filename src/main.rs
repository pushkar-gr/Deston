mod config;
mod load_balancer;
mod server;
use crate::load_balancer::load_balancer::LoadBalancer;
use config::config::{Config, LayerMode};
use load_balancer::layer4::Layer4;
use load_balancer::layer7::Layer7;
use std::path::Path;

use std::sync::{Arc, Mutex};
use tokio::signal;

#[tokio::main]
async fn main() {
    let config = Config::new(Path::new("config.toml"));
    let layer_mode = config.layer_mode.clone();
    let config_arc = Arc::new(Mutex::new(config));

    // Create a channel for graceful shutdown
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    // Spawn a task to handle shutdown signals
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                println!("\nReceived shutdown signal, initiating graceful shutdown...");
                let _ = shutdown_tx.send(true);
            }
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
            }
        }
    });

    match layer_mode {
        LayerMode::L4 => {
            println!("Starting Layer 4 (TCP) Load Balancer...");
            let lb = Layer4::new(config_arc);
            if let Err(e) = lb.start(shutdown_rx).await {
                eprintln!("Error starting Layer 4 load balancer: {:?}", e);
            }
        }
        LayerMode::L7 => {
            println!("Starting Layer 7 (HTTP) Load Balancer...");
            let lb = Layer7::new(config_arc);
            if let Err(e) = lb.start(shutdown_rx).await {
                eprintln!("Error starting Layer 7 load balancer: {:?}", e);
            }
        }
    }

    println!("Load balancer shut down gracefully.");
}
