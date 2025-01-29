use hyper::Uri;
use tokio::io::{copy, split, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::try_join;

use crate::load_balancer::load_balancer;

pub struct Layer4;

impl Layer4 {
    //establishes connection with server and transfers data between server and client
    async fn transfer_data(client_stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        //server address
        //!todo: pick the server based on an algorithm
        let server_address = "http://127.0.0.1:3001".parse::<Uri>().unwrap();
        let host = server_address.host().unwrap();
        let port = server_address.port_u16().unwrap();

        //creates a new server stream
        let server_stream = TcpStream::connect((host, port)).await?;

        //splits server and client streams into read and write streams
        let (mut client_read, mut client_write) = split(client_stream);
        let (mut server_read, mut server_write) = split(server_stream);

        //transfers data from client to server
        let client_to_server = tokio::spawn(async move {
            copy(&mut client_read, &mut server_write).await?;
            server_write.shutdown().await
        });

        //transfers data from server to client
        let server_to_server = tokio::spawn(async move {
            copy(&mut server_read, &mut client_write).await?;
            client_write.shutdown().await
        });

        //runs both diretions concurrently
        let _ = try_join!(client_to_server, server_to_server)?;

        Ok(())
    }
}

impl load_balancer::LoadBalancer for Layer4 {
    //creates and returns a new Layer4 load balancer
    fn new() -> Self {
        Layer4
    }

    //start_layer4 starts layer 4 load balancer
    //accepts incoming request at given address and calls transfer_data to transfer data between server and client
    async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let lb_address = "http://127.0.0.1:8000".parse::<Uri>().unwrap();
        let host = lb_address.host().unwrap();
        let port = lb_address.port_u16().unwrap();

        //create a TcpListener and binds it to load balancer address
        let listener = TcpListener::bind((host, port)).await?;

        //loop to continuously accetp incoming connections
        loop {
            let (stream, _) = listener.accept().await?;

            //spawn a tokio task to server multiple connections concurrently
            tokio::task::spawn(async move {
                if let Err(err) = Layer4::transfer_data(stream).await {
                    eprintln!("Error transfering data {:?}", err);
                }
            });
        }
    }

    fn stop(&self) {}
}
