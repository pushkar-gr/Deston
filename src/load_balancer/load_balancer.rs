//defines load balancer trait, it provies the blueprint for creating, starting and stopping load balancer. With a method to pick the next server based on given algo

use std::sync::{Arc, Mutex};

use crate::server::server::{SyncServer, SyncServers};

pub trait LoadBalancer {
    //returns a LoadBalancer
    fn new(servers: SyncServers) -> Self;

    //starts the load balancer
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    //picks a server based on algo to handle incoming request
    //returns an option of server if server available
    //returns None if no servers are available
    async fn pick_server(servers: SyncServers) -> Option<SyncServer> {
        //lock servers to perform actions
        let locked_servers = servers.lock().unwrap();

        //pick a server
        //*!todo: use specified algorithm to pick the server
        Some(locked_servers[0].clone())
    }

    //stops the load balancer
    fn stop(&self);
}
