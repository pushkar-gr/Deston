//defines algorithm trait, it provies the blueprint for creating, and picking servers

use crate::load_balancer::load_balancer::PickServerError;
use crate::server::server::SyncServer;
use std::net::SocketAddr;
use std::sync::Arc;

pub trait Algorithm: Send {
    //returns new Algorithm struct
    fn new() -> Self
    where
        Self: Sized;

    //picks server based on algorithm and returns server index and server. Returns error if any
    fn pick_server(
        &mut self,
        servers: Arc<Vec<SyncServer>>,
        client_addr: SocketAddr,
    ) -> Result<(usize, SyncServer), PickServerError>;
}
