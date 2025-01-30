use std::sync::{Arc, Mutex};
//basic structure
struct RoundRobin {
    servers: Vec<String>,
    idx: Mutex<usize>,
}

impl RoundRobin{
    fn new(servers: Vec<String>) -> Arc<Self> {
        Arc::new(Self {
            servers,
            idx: Mutex::new(0),
        })
    }
//basic round-robin routing
    fn next_server(&self) -> String {
        let mut idx = self.idx.lock().unwrap();
        let server = self.servers[*idx].clone();
        *idx = (*idx + 1) % self.servers.len();
        server
    }
}



