//defines the Layer 7 Load Balancer that implements the LoadBalancer trait. It listens for incoming HTTP requests, selects an appropriate server, and forwards the requests to the chosen server

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Uri;
use hyper_util::rt::TokioIo;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::load_balancer::load_balancer::LoadBalancer;
use crate::server::server::{Server, SyncServer, SyncServers};

pub struct Layer7 {
    servers: SyncServers,
}

impl LoadBalancer for Layer7 {
    //creates and returns a new Layer7 load balancer
    fn new(servers: SyncServers) -> Self {
        Layer7 { servers }
    }

    //starts layer 7 load balancer
    //will listen to incoming requests at given address
    //calls pick_server to pick a server when user sends a request
    //calls Server::handle_request to forward request to the server
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
            let io = TokioIo::new(stream);

            //clone the server list to safely share across multiple threads
            let servers_clone = self.servers.clone();

            //spawn a tokio task to server multiple connections concurrently
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .preserve_header_case(true)
                    .title_case_headers(true)
                    //bind the incoming connection to handle_request
                    .serve_connection(
                        io,
                        service_fn(move |req| {
                            //clone the server list to safely share across multiple threads
                            let servers_clone = servers_clone.clone();
                            async move {
                                //pick a server
                                let server =
                                    Layer7::pick_server(servers_clone).await.expect("No server");
                                //call Server::handle_request to forward the request to server
                                Server::handle_request(server, req, addr).await
                            }
                        }),
                    )
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    //stops layer 7 load balancer
    fn stop(&self) {}
}
