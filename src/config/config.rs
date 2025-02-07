//defines Config struct that holds list of structures, selected load balancer algorithm, and an algorithm object.

use std::sync::{Arc, Mutex};

use crate::load_balancer::algorithm::algorithm::Algorithm as AlgorithmTrait;
use crate::load_balancer::algorithm::r#static::{
    ip_hashing::IpHashing, round_robin::RoundRobin, weighted_round_robin::WeightedRoundRobin,
};
use crate::server::server::SyncServer;

//type alias for a thread-safe, synchronized Config using Arc and Mutex
pub type SyncConfig = Arc<Mutex<Config>>;

//enum for all the algorithm
#[derive(Clone)]
pub enum Algorithm {
    RoundRobin,         //round robin
    WeightedRoundRobin, //weighted round robin
    IpHashing,          //ip hashing
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
            algorithm: algorithm.clone(),
            algorithm_object: {
                //pick algorithm based on input
                match algorithm {
                    Algorithm::RoundRobin => Box::new(RoundRobin::new()),
                    Algorithm::WeightedRoundRobin => Box::new(WeightedRoundRobin::new()),
                    Algorithm::IpHashing => Box::new(IpHashing::new()),
                }
            },
        }
    }
}
