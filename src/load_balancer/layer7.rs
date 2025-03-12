//defines the Layer 7 Load Balancer that implements the LoadBalancer trait. It listens for incoming HTTP requests, selects an appropriate server, and forwards the requests to the chosen server

use crate::config::config::SyncConfig;
use crate::load_balancer::load_balancer::LoadBalancer;
use crate::server::server::Server;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub struct Layer7 {
    config: SyncConfig,
}

impl LoadBalancer for Layer7 {
    //creates and returns a new Layer7 load balancer
    fn new(config: SyncConfig) -> Self {
        Self { config }
    }

    //starts layer 7 load balancer
    //will listen to incoming requests at given address
    //calls pick_server to pick a server when user sends a request
    //calls Server::handle_request to forward request to the server
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        //load balancer address
        let lb_address = self.config.lock().unwrap().load_balancer_address.clone();
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
            let config_clone = self.config.clone();

            //spawn a tokio task to server multiple connections concurrently
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .preserve_header_case(true)
                    .title_case_headers(true)
                    //bind the incoming connection to handle_request
                    .serve_connection(
                        io,
                        service_fn(move |req: Request<Incoming>| {
                            //clone the server list to safely share across multiple threads
                            let config_clone = config_clone.clone();
                            async move {
                                //convert request<Incoming> to request<Full<Bytes>>
                                let (parts, body) = req.into_parts();
                                let body = body.collect().await?.to_bytes();
                                let req = Request::from_parts(parts, Full::new(body));

                                //run loop for fault tolerance
                                loop {
                                    //pick a server
                                    let config_clone = config_clone.clone();
                                    let server = Self::pick_server(config_clone.clone(), addr)
                                        .await
                                        .expect("No server");

                                    //call Server::handle_request to forward the request to server
                                    let res =
                                        Server::handle_request(server, req.clone(), addr).await;
                                    //return response if succuss, else pick new server
                                    if res.is_ok() {
                                        return res;
                                    }
                                }
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
