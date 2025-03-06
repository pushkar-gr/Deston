//defines weighted round robin algorithm, where servers are selected sequentially, taking their assigned weights into account. Servers with higher weights are picked more frequently

use crate::load_balancer::algorithm::algorithm::Algorithm;
use crate::load_balancer::load_balancer::PickServerError;
use crate::server::server::SyncServer;
use std::net::SocketAddr;
use std::sync::Arc;

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
    ) -> Result<(usize, SyncServer), PickServerError> {
        loop {
            //get server
            let server = &servers[self.index];
            //get weight of server
            let server_weight = { server.lock()?.weight };

            if self.curr_weight < server_weight {
                self.curr_weight += 1;
                return Ok((self.index, server.clone()));
            }
            //reset weight and move to next server
            self.curr_weight = 0;
            self.index = (self.index + 1) % servers.len();
        }
    }
}
