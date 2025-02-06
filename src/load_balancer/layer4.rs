//defines the Layer4 load balancer that implements the LoadBalancer trait. It manages multiple backend servers and handles Layer 4 (transport layer) requests. The load balancer listens for incoming connections, picks an appropriate server, and transfers data between the client and the selected server

use hyper::Uri;
use tokio::net::TcpListener;

use crate::config::config::SyncConfig;
use crate::load_balancer::load_balancer;
use crate::server::server::Server;

pub struct Layer4 {
    config: SyncConfig,
}

impl load_balancer::LoadBalancer for Layer4 {
    //creates and returns a new Layer4 load balancer
    fn new(config: SyncConfig) -> Self {
        Self { config }
    }

    //starts layer 4 load balancer
    //will listen to incoming requests at given address
    //calls pick_server to pick a server when user sends a request
    //calls Server::transfer_data to transfer data between server and client
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        //load balancer address
        let lb_address = "http://127.0.0.1:8000".parse::<Uri>().unwrap();
        let host = lb_address.host().unwrap();
        let port = lb_address.port_u16().unwrap();

        //create a TcpListener and binds it to load balancer address
        let listener = TcpListener::bind((host, port)).await?;

        //loop to continuously accetp incoming connections
        loop {
            //accept incoming connections
            let (stream, addr) = listener.accept().await?;

            //clone the server list to safely share across multiple threads
            let config_clone = self.config.clone();

            //spawn a tokio task to server multiple connections concurrently
            tokio::task::spawn(async move {
                //pick a server
                let server = Self::pick_server(config_clone, addr)
                    .await
                    .expect("No server");
                //call Server::transfer_data to transfer data between server and client
                if let Err(err) = Server::transfer_data(server, stream).await {
                    eprintln!("Error transfering data {:?}", err);
                }
            });
        }
    }

    //stops layer 4 load balancer
    fn stop(&self) {}
}
