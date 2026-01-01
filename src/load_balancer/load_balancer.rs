//! Load balancer trait and implementations.
//!
//! This module defines the core LoadBalancer trait and provides implementations
//! for Layer 4 (TCP) and Layer 7 (HTTP) load balancing.

use std::net::SocketAddr;

use crate::config::config::SyncConfig;
use crate::server::server::SyncServer;

/// LoadBalancer trait defining the interface for load balancer implementations
#[allow(async_fn_in_trait)]
pub trait LoadBalancer {
    /// Creates a new LoadBalancer with the given configuration
    fn new(config: SyncConfig) -> Self;

    /// Starts the load balancer and begins accepting connections
    ///
    /// # Arguments
    /// * `shutdown_rx` - A watch receiver that signals when to initiate shutdown
    async fn start(
        &self,
        shutdown_rx: tokio::sync::watch::Receiver<bool>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Picks a server based on the configured algorithm to handle an incoming request
    ///
    /// Returns Some(server) if a server is available, None otherwise
    async fn pick_server(config: SyncConfig, client_addr: SocketAddr) -> Option<SyncServer> {
        //lock config
        let mut config = config.lock().unwrap();
        //get servers
        let servers = config.servers.clone();
        //call Algorithm::pick_server and return the server
        let (index, server) = config
            .algorithm_object
            .pick_server(servers, client_addr)
            .unwrap();
        //update index
        config.last_picked_index = index;
        //return picked server
        Some(server)
    }
}
