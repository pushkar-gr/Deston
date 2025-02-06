use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::load_balancer::algorithm::algorithm::Algorithm;
use crate::server::server::SyncServer;

pub struct IpHashing {}

impl Algorithm for IpHashing {
    //creates and returns new RoundRobin
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
    ) -> Option<(usize, SyncServer)> {
        //create hasher
        let mut hasher = Sha256::new();
        //hash client_addr
        hasher.update(client_addr.to_string().as_bytes());
        let result = hasher.finalize();
        //get index from result
        let index =
            (usize::from_be_bytes(result[0..8].try_into().unwrap())) % servers.len();
        //return index and server
        Some((index, servers[index].clone()))
    }
}
