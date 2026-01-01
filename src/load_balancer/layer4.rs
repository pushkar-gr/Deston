//! Layer 4 (TCP) load balancer implementation.
//!
//! This module provides a Layer 4 load balancer that operates at the transport layer,
//! forwarding raw TCP connections between clients and backend servers.

use tokio::net::TcpListener;

use crate::config::config::SyncConfig;
use crate::load_balancer::load_balancer;
use crate::server::server::Server;

/// Layer 4 (TCP) Load Balancer
pub struct Layer4 {
    config: SyncConfig,
}

impl load_balancer::LoadBalancer for Layer4 {
    //creates and returns a new Layer4 load balancer
    fn new(config: SyncConfig) -> Self {
        Self { config }
    }

    //starts layer 4 load balancer
    //will listen to incoming requests at given address
    //calls pick_server to pick a server when user sends a request
    //calls Server::transfer_data to transfer data between server and client
    async fn start(
        &self,
        mut shutdown_rx: tokio::sync::watch::Receiver<bool>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        //load balancer address from config
        let lb_address = {
            let config = self.config.lock().unwrap();
            config.load_balancer_address.clone()
        };
        let host = lb_address.host().unwrap();
        let port = lb_address.port_u16().unwrap();

        //create a TcpListener and binds it to load balancer address
        let listener = TcpListener::bind((host, port)).await?;

        println!("Layer 4 Load Balancer listening on {}:{}", host, port);

        //loop to continuously accept incoming connections
        loop {
            tokio::select! {
                // Check if shutdown signal is received
                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        println!("Shutdown signal received, stopping Layer 4 Load Balancer...");
                        break;
                    }
                }
                // Accept incoming connections
                result = listener.accept() => {
                    match result {
                        Ok((stream, addr)) => {
                            //clone the server list to safely share across multiple threads
                            let config_clone = self.config.clone();

                            //spawn a tokio task to server multiple connections concurrently
                            tokio::task::spawn(async move {
                                //pick a server
                                let server = Self::pick_server(config_clone, addr)
                                    .await
                                    .expect("No server");
                                //call Server::transfer_data to transfer data between server and client
                                if let Err(err) = Server::transfer_data(server, stream).await {
                                    eprintln!("Error transferring data {:?}", err);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Error accepting connection: {:?}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
