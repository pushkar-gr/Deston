//defines load balancer trait, it provies the blueprint for creating, starting and stopping load balancer. With a method to pick the next server based on given algo

use std::net::SocketAddr;

use crate::config::config::SyncConfig;
use crate::server::server::SyncServer;

pub trait LoadBalancer {
    //returns a LoadBalancer
    fn new(config: SyncConfig) -> Self;

    //starts the load balancer
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    //picks a server based on algo to handle incoming request
    //returns an option of server if server available
    //returns None if no servers are available
    async fn pick_server(config: SyncConfig, client_addr: SocketAddr) -> Option<SyncServer> {
        //lock config
        let mut config = config.lock().unwrap();
        //get servers
        let servers = config.servers.clone();
        //call Algorithm::pick_server and return the server
        let (index, server) = config
            .algorithm_object
            .pick_server(servers, client_addr)
            .unwrap();
        //update index
        config.last_picked_index = index;
        //return picked server
        Some(server)
    }

    //stops the load balancer
    #[allow(dead_code)]
    fn stop(&self);
}
