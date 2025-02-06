use crate::Arc;

use crate::server::server::SyncServer;

pub trait Algorithm: Send {
    //returns new Algorithm struct
    fn new() -> Self
    where
        Self: Sized;

    //picks server based on algorithm and returns server index and server. returns None if no server available
    fn pick_server(&mut self, servers: Arc<Vec<SyncServer>>) -> Option<(usize, SyncServer)>;
}
