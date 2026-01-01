//! Layer 7 (HTTP) load balancer implementation.
//!
//! This module provides a Layer 7 load balancer that operates at the application layer,
//! forwarding HTTP requests with the ability to inspect and modify headers.

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::config::config::SyncConfig;
use crate::load_balancer::load_balancer::LoadBalancer;
use crate::server::server::Server;

/// Layer 7 (HTTP) Load Balancer
#[allow(dead_code)]
pub struct Layer7 {
    config: SyncConfig,
}

impl LoadBalancer for Layer7 {
    //creates and returns a new Layer7 load balancer
    fn new(config: SyncConfig) -> Self {
        Self { config }
    }

    //starts layer 7 load balancer
    //will listen to incoming requests at given address
    //calls pick_server to pick a server when user sends a request
    //calls Server::handle_request to forward request to the server
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

        println!("Layer 7 Load Balancer listening on {}:{}", host, port);

        //loop to continuously accept incoming connections
        loop {
            tokio::select! {
                // Check if shutdown signal is received
                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        println!("Shutdown signal received, stopping Layer 7 Load Balancer...");
                        break;
                    }
                }
                // Accept incoming connections
                result = listener.accept() => {
                    match result {
                        Ok((stream, addr)) => {
                            let io = TokioIo::new(stream);

                            //clone the server list to safely share across multiple threads
                            let config_clone = self.config.clone();

                            //spawn a tokio task to server multiple connections concurrently
                            tokio::task::spawn(async move {
                                if let Err(err) = http1::Builder::new()
                                    .preserve_header_case(true)
                                    .title_case_headers(true)
                                    //bind the incoming connection to handle_request
                                    .serve_connection(
                                        io,
                                        service_fn(move |req| {
                                            //clone the server list to safely share across multiple threads
                                            let config_clone = config_clone.clone();
                                            async move {
                                                //pick a server
                                                let config_clone = config_clone.clone();
                                                let server = Self::pick_server(config_clone, addr)
                                                    .await
                                                    .expect("No server");
                                                //call Server::handle_request to forward the request to server
                                                Server::handle_request(server, req, addr).await
                                            }
                                        }),
                                    )
                                    .await
                                {
                                    eprintln!("Error serving connection: {:?}", err);
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
