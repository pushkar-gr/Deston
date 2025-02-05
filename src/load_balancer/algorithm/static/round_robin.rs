use std::sync::MutexGuard;

use crate::load_balancer::algorithm::algorithm;
use crate::server::server::SyncServer;

pub struct RoundRobin {
    index: usize,
}

impl algorithm::Algorithm for RoundRobin {
    //creates and returns new RoundRobin
    fn new() -> Self
    where
        Self: Sized,
    {
        RoundRobin { index: 0 }
    }

    //picks next server
    //picks server at index, increments index and returns the index and server
    fn pick_server(&mut self, servers: MutexGuard<Vec<SyncServer>>) -> Option<(usize, SyncServer)> {
        //pick server
        let server = servers[self.index].clone();
        let index = self.index;
        //incriment index
        self.index = (self.index + 1) % servers.len();
        //return index and server
        Some((index, server))
    }
}
