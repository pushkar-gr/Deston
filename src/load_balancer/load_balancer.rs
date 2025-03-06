//defines load balancer trait, it provies the blueprint for creating, starting and stopping load balancer. With a method to pick the next server based on given algo

use crate::config::config::SyncConfig;
use crate::server::server::SyncServer;
use std::array::TryFromSliceError;
use std::net::SocketAddr;
use std::sync::{MutexGuard, PoisonError};
use thiserror::Error;

//enum for errors returned by pick_server method
#[derive(Debug, Error)]
pub enum PickServerError {
    #[error("Mutex lock poisoned: {0}")]
    MutexPoisonError(String),

    #[error("Failed to convert bytes to usize: {0}")]
    ConversionError(#[from] TryFromSliceError),
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for PickServerError {
    fn from(err: PoisonError<MutexGuard<'_, T>>) -> Self {
        PickServerError::MutexPoisonError(err.to_string())
    }
}

pub trait LoadBalancer {
    //returns a LoadBalancer
    fn new(config: SyncConfig) -> Self;

    //starts the load balancer
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    //picks a server based on algo to handle incoming request
    //returns server if server available and no error occured
    //returns PickServerError else
    async fn pick_server(
        config: SyncConfig,
        client_addr: SocketAddr,
    ) -> Result<SyncServer, PickServerError> {
        //lock config
        let mut config = config.lock()?;
        //get servers
        let servers = config.servers.clone();
        //call AlgoRithm::pick_server and return the server
        let (index, server) = config.algorithm_object.pick_server(servers, client_addr)?;
        //update index
        config.last_picked_index = index;
        //return picked server
        Ok(server)
    }

    //stops the load balancer
    fn stop(&self);
}
