//! Configuration module for the Deston load balancer.
//!
//! This module handles parsing and managing configuration from TOML files.
//! It defines the configuration structure including load balancer settings,
//! backend servers, and algorithm selection.

use hyper::Uri;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use toml::{Table, Value};

use crate::load_balancer::algorithm::algorithm::Algorithm as AlgorithmTrait;
use crate::load_balancer::algorithm::r#static::{
    ip_hashing::IpHashing, round_robin::RoundRobin, weighted_round_robin::WeightedRoundRobin,
};
use crate::server::server::{Server, SyncServer};

//type alias for a thread-safe, synchronized Config using Arc and Mutex
pub type SyncConfig = Arc<Mutex<Config>>;

/// Load balancing algorithm options
#[derive(Clone)]
pub enum Algorithm {
    RoundRobin,         //round robin
    WeightedRoundRobin, //weighted round robin
    IpHashing,          //ip hashing
}

/// Configuration structure for the load balancer
pub struct Config {
    pub load_balancer_address: Uri,    //address of load balancer
    pub servers: Arc<Vec<SyncServer>>, //thread safe vector of servers
    #[allow(dead_code)]
    pub algorithm: Algorithm, //algorithm to pick server
    pub last_picked_index: usize,      //index of last picked server
    pub algorithm_object: Box<dyn AlgorithmTrait>, //algorithm object
}

impl Config {
    /// Creates and returns a new Config from a TOML file
    pub fn new(config_path: &Path) -> Self {
        //read contents of config file
        let contents = fs::read_to_string(config_path).unwrap();
        //parse config file contents
        let values = contents.parse::<Table>().unwrap();

        //get host name, port and algorithm of load balancer
        let (load_balancer_host, load_balancer_port, algorithm) = {
            if let Some(table) = values.get("load_balancer") {
                let host = {
                    if let Some(Value::String(address)) = table.get("address") {
                        address.as_str()
                    } else {
                        //if host not found in config
                        "localhost"
                    }
                };
                let port = {
                    if let Some(Value::Integer(port)) = table.get("port") {
                        port
                    } else {
                        &8080
                    }
                };
                let algorithm = {
                    if let Some(Value::String(algorithm)) = table.get("algorithm") {
                        get_algorithm(algorithm)
                    } else {
                        Algorithm::RoundRobin
                    }
                };
                (host, port, algorithm)
            } else {
                //if host not found in config
                ("localhost", &8080, Algorithm::RoundRobin)
            }
        };

        //create Config
        Self {
            //address of load balancer
            load_balancer_address: (load_balancer_host.to_owned()
                + ":"
                + &load_balancer_port.to_string())
                .parse::<Uri>()
                .unwrap(),

            //list of servers
            servers: {
                //get servers table
                if let Some(Value::Array(servers)) = values.get("server") {
                    Arc::new(
                        servers
                            .iter()
                            .map(|server| {
                                //get server host
                                let server_host = {
                                    if let Some(Value::String(address)) = server.get("address") {
                                        address.as_str()
                                    } else {
                                        "localhost"
                                    }
                                };
                                //get server port
                                let server_port = {
                                    if let Some(Value::Integer(port)) = server.get("port") {
                                        port
                                    } else {
                                        &3000
                                    }
                                };
                                //get max connections
                                let max_connections = *{
                                    if let Some(Value::Integer(max_connections)) =
                                        server.get("max_connections")
                                    {
                                        max_connections
                                    } else {
                                        &1000
                                    }
                                } as u32;
                                //get weight
                                let weight = *{
                                    if let Some(Value::Integer(weight)) = server.get("weight") {
                                        weight
                                    } else {
                                        &1
                                    }
                                } as usize;
                                //create new server object
                                Arc::new(Mutex::new(Server::new(
                                    (server_host.to_owned() + ":" + &server_port.to_string())
                                        .parse::<Uri>()
                                        .unwrap(),
                                    max_connections,
                                    weight,
                                )))
                            })
                            .collect(),
                    )
                } else {
                    //if servers not found in config
                    let server1 =
                        Server::new("http://127.0.0.1:3000".parse::<Uri>().unwrap(), 1000, 1);
                    let server2 =
                        Server::new("http://127.0.0.1:3001".parse::<Uri>().unwrap(), 1000, 1);
                    Arc::new(vec![
                        Arc::new(Mutex::new(server1)),
                        Arc::new(Mutex::new(server2)),
                    ])
                }
            },
            algorithm_object: {
                //pick algorithm based on input
                match &algorithm {
                    Algorithm::RoundRobin => Box::new(RoundRobin::new()),
                    Algorithm::WeightedRoundRobin => Box::new(WeightedRoundRobin::new()),
                    Algorithm::IpHashing => Box::new(IpHashing::new()),
                }
            },
            algorithm: algorithm,
            last_picked_index: 0,
        }
    }
}

//function to get Algorithm from string (case-insensitive)
fn get_algorithm(algorithm: &String) -> Algorithm {
    let algo_lower = algorithm.to_lowercase();
    if algo_lower == "roundrobin" || algo_lower == "round_robin" {
        Algorithm::RoundRobin
    } else if algo_lower == "weightedroundrobin" || algo_lower == "weighted_round_robin" {
        Algorithm::WeightedRoundRobin
    } else if algo_lower == "iphashing" || algo_lower == "ip_hashing" {
        Algorithm::IpHashing
    } else {
        Algorithm::RoundRobin
    }
}
