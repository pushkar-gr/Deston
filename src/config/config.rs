//defines Config struct that holds list of structures, selected load balancer algorithm, and an algorithm object.

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

//enum for all the algorithm
#[derive(Clone)]
pub enum Algorithm {
    RoundRobin,         //round robin
    WeightedRoundRobin, //weighted round robin
    IpHashing,          //ip hashing
}

pub struct Config {
    pub load_balancer_address: Uri,    //address of load balancer
    pub servers: Arc<Vec<SyncServer>>, //thread safe vector of servers
    pub algorithm: Algorithm,          //algorithm to pick server
    pub last_picked_index: usize,      //index of last picked server
    pub algorithm_object: Box<dyn AlgorithmTrait>, //algorithm object
}

impl Config {
    //creates and returns a new Config
    pub fn new(config_path: &Path) -> Self {
        //read contents of config file
        let contents = fs::read_to_string(config_path).unwrap();
        //parese config file contents
        let values = contents.parse::<Table>().unwrap();

        //get host name of load balancer
        let load_balancer_host = {
            if let Value::String(address) =
                values.get("load_balancer").unwrap().get("address").unwrap()
            {
                address
            } else {
                //if host not found in config
                &"localhost".to_string()
            }
        };
        //get port of load balancer
        let load_balancer_port = {
            if let Value::Integer(port) = values.get("load_balancer").unwrap().get("port").unwrap()
            {
                port
            } else {
                //if port not found in config
                &8080
            }
        };
        //get algorithm for load balancer
        let algorithm: Algorithm = {
            if let Value::String(algo) = values
                .get("load_balancer")
                .unwrap()
                .get("algorithm")
                .unwrap()
            {
                get_algorithm(algo)
            } else {
                //if algorithm not found in config
                Algorithm::RoundRobin
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
                if let Value::Array(servers) = values.get("server").unwrap() {
                    Arc::new(
                        servers
                            .iter()
                            .map(|server| {
                                //get server host
                                let server_host = {
                                    if let Value::String(address) = server.get("address").unwrap() {
                                        address
                                    } else {
                                        &"localhost".to_string()
                                    }
                                };
                                //get server port
                                let server_port = {
                                    if let Value::Integer(port) = server.get("port").unwrap() {
                                        port
                                    } else {
                                        &3000
                                    }
                                };
                                //create new server object
                                Arc::new(Mutex::new(Server::new(
                                    (server_host.to_owned() + ":" + &server_port.to_string())
                                        .parse::<Uri>()
                                        .unwrap(),
                                )))
                            })
                            .collect(),
                    )
                } else {
                    //if servers not found in config
                    let server1 = Server::new("http://127.0.0.1:3000".parse::<Uri>().unwrap());
                    let server2 = Server::new("http://127.0.0.1:3001".parse::<Uri>().unwrap());
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

//funciton to get Algorithm from string
fn get_algorithm(algorithm: &String) -> Algorithm {
    if algorithm == "RoundRobin" {
        Algorithm::RoundRobin
    } else if algorithm == "WeightedRoundRobin" {
        Algorithm::WeightedRoundRobin
    } else if algorithm == "IpHashing" {
        Algorithm::IpHashing
    } else {
        Algorithm::RoundRobin
    }
}
