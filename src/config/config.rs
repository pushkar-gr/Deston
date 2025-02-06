use std::sync::{Arc, Mutex};

use crate::load_balancer::algorithm::algorithm::Algorithm as AlgorithmTrait;
use crate::load_balancer::algorithm::r#static::round_robin::RoundRobin;
use crate::load_balancer::algorithm::r#static::ip_hashing::IpHashing;
use crate::server::server::SyncServer;

//type alias for a thread-safe, synchronized Config using Arc and Mutex
pub type SyncConfig = Arc<Mutex<Config>>;

//enum for all the algorithm
pub enum Algorithm {
    RoundRobin, //round robin
}

pub struct Config {
    pub servers: Arc<Vec<SyncServer>>, //thread safe vector of servers
    pub algorithm: Algorithm,          //algorithm to pick server
    pub last_picked_index: usize,      //index of last picked server
    pub algorithm_object: Box<dyn AlgorithmTrait>, //algorithm object
}

impl Config {
    //creates and returns a new Config
    pub fn new(servers: Arc<Vec<SyncServer>>, algorithm: Algorithm) -> Self {
        Self {
            last_picked_index: servers.len(),
            servers: servers,
            algorithm: algorithm,
            algorithm_object: Box::new(IpHashing::new()),
        }
    }
}
