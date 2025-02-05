use std::sync::{Arc, Mutex};

use crate::load_balancer::algorithm::algorithm::Algorithm as AlgorithmTrait;
use crate::load_balancer::algorithm::r#static::round_robin::RoundRobin;
use crate::server::server::SyncServers;

//type alias for a thread-safe, synchronized Config using Arc and Mutex
pub type SyncConfig = Arc<Mutex<Config>>;

//enum for all the algorithm
pub enum Algorithm {
    RoundRobin, //round robin
}

pub struct Config {
    pub servers: SyncServers,     //thread safe vector of servers
    pub algorithm: Algorithm,     //algorithm to pick server
    pub last_picked_index: usize, //index of last picked server
    pub algorithm_object: Arc<Mutex<dyn AlgorithmTrait>>, //thread safe algorithm object
}

impl Config {
    //creates and returns a new Config
    pub fn new(servers: SyncServers, algorithm: Algorithm) -> Self {
        Config {
            servers: servers.clone(),
            algorithm: algorithm,
            last_picked_index: servers.lock().unwrap().len(),
            algorithm_object: Arc::new(Mutex::new(RoundRobin::new())),
        }
    }
}
