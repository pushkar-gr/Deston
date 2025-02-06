
use std::sync::{Arc, Mutex};
//basic structure
struct Server {
    name: String,
    weight: usize,
}
//basic structure:weight_rounded_robin
struct Weight_rr {
    servers: Vec<Server>,
    idx: Mutex<usize>,
    curr_weight: Mutex<usize>,
}

impl Weight_rr {
    fn new(servers: Vec<(String, usize)>) -> Arc<Self> {
        let servers = servers.into_iter()
            .map(|(name, weight)| Server { name, weight })
            .collect();
        Arc::new(Self {
            servers,
            idx: Mutex::new(0),
            curr_weight: Mutex::new(0),
        })
    }
//Servers with higher weights are given a larger proportion of the requests.
    fn next_server(&self) -> String {
        let mut idx = self.idx.lock().unwrap();
        let mut curr_weight = self.curr_weight.lock().unwrap();

        loop {
            let server = &self.servers[*idx];   
            if *curr_weight < server.weight {
                *curr_weight += 1;
                return server.name.clone();
            }
            *curr_weight = 0;
            *idx = (*idx + 1) % self.servers.len();
        }
    }
}
