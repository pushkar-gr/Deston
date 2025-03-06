//defines ip hashing, where servers are selected based on client ip address

use crate::load_balancer::algorithm::algorithm::Algorithm;
use crate::load_balancer::load_balancer::PickServerError;
use crate::server::server::SyncServer;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use std::sync::Arc;

pub struct IpHashing {}

impl Algorithm for IpHashing {
    //creates and returns new IpHashing
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    //picks next server
    //hashes client ip address, picks and returns resultig server
    fn pick_server(
        &mut self,
        servers: Arc<Vec<SyncServer>>,
        client_addr: SocketAddr,
    ) -> Result<(usize, SyncServer), PickServerError> {
        //create hasher
        let mut hasher = Sha256::new();
        //hash client_addr
        hasher.update(client_addr.to_string().as_bytes());
        let result = hasher.finalize();
        //get index from result
        let index = (usize::from_be_bytes(result[0..8].try_into()?)) % servers.len();
        //return index and server
        Ok((index, servers[index].clone()))
    }
}
