use std::sync::{Arc, Mutex};

struct RoundRobin {
    servers: Vec<String>,
    index: Mutex<usize>,
}

impl RoundRobin{
    fn new(servers: Vec<String>) -> Arc<Self> {
        Arc::new(Self {
            servers,
            index: Mutex::new(0),
        })
    }

    fn next_server(&self) -> String {
        let mut index = self.index.lock().unwrap();
        let server = self.servers[*index].clone();
        *index = (*index + 1) % self.servers.len();
        server
    }
}



