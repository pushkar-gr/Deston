//defines round robin algorithm, where servers are selected sequentially

use crate::Arc;
use std::net::SocketAddr;

use crate::load_balancer::algorithm::algorithm::Algorithm;
use crate::server::server::SyncServer;

pub struct RoundRobin {
    index: usize,
}

impl Algorithm for RoundRobin {
    //creates and returns new RoundRobin
    fn new() -> Self
    where
        Self: Sized,
    {
        Self { index: 0 }
    }

    //picks next server
    //picks server at index, increments index and returns the index and server
    fn pick_server(
        &mut self,
        servers: Arc<Vec<SyncServer>>,
        _: SocketAddr,
    ) -> Option<(usize, SyncServer)> {
        //pick server
        let server = servers[self.index].clone();
        let index = self.index;
        //incriment index
        self.index = (self.index + 1) % servers.len();
        //return index and server
        Some((index, server))
    }
}
