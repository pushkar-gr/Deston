//! Algorithm trait definition.
//!
//! This module defines the Algorithm trait that all load balancing algorithms must implement.

use crate::Arc;
use std::net::SocketAddr;

use crate::server::server::SyncServer;

/// Algorithm trait for load balancing strategies
pub trait Algorithm: Send {
    /// Creates a new instance of the algorithm
    fn new() -> Self
    where
        Self: Sized;

    /// Picks a server based on the algorithm's strategy
    ///
    /// Returns Some((index, server)) if a server is available, None otherwise
    fn pick_server(
        &mut self,
        servers: Arc<Vec<SyncServer>>,
        client_addr: SocketAddr,
    ) -> Option<(usize, SyncServer)>;
}
