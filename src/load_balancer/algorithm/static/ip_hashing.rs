use sha2::{Sha256, Digest};
use std::sync::Arc;

struct IpHashing {
    servers: Vec<String>,
}

impl IpHashing {
    fn new(servers: Vec<String>) -> Arc<Self> {
        Arc::new(Self { servers })
    }

    fn get_server(&self, ip: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(ip.as_bytes());
        let result = hasher.finalize();
        let index = (u64::from_be_bytes(result[0..8].try_into().unwrap()) as usize) % self.servers.len();
        self.servers[index].clone()
    }
}
