use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::load_balancer::algorithm::algorithm::Algorithm;
use crate::server::server::SyncServer;

pub struct WeightedRoundRobin {
    index: usize,
    curr_weight: usize,
}

impl Algorithm for WeightedRoundRobin {
    //creates and returns new WeightedRoundRobin
    fn new() -> Self {
        Self {
            index: 0,
            curr_weight: 0,
        }
    }

    //picks next server
    //picks server at index wtr to weight, increments index and returns the index and server
    fn pick_server(
        &mut self,
        servers: Arc<Vec<SyncServer>>,
        _: SocketAddr,
    ) -> Option<(usize, SyncServer)> {
        loop {
            //get server
            let server = &servers[self.index];
            //get weight of server
            let server_weight = { server.lock().unwrap().weight };

            if self.curr_weight < server_weight {
                self.curr_weight += 1;
                return Some((self.index, server.clone()));
            }
            //reset weight and move to next server
            self.curr_weight = 0;
            self.index = (self.index + 1) % servers.len();
        }
    }
}
