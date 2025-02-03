use http::header::{HeaderValue, FORWARDED};
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::body::{Bytes, Incoming};
use hyper::client::conn::http1::Builder;
use hyper::{Request, Response, Uri};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio::io::{copy, split, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::try_join;

//type alias for a thread-safe, synchronized Server using Arc and Mutex
pub type SyncServer = Arc<Mutex<Server>>;

#[derive(Clone)]
pub struct Server {
    uri: Uri,     //uri of server
    host: String, //hostname of server
    port: u16,    //port at which server is running

    max_connections: u32,        //max connections server can handle
    connections: u32,            //number of alive connections
    total_connections: u32,      //total connections server has served
    successful_connections: u32, //total successful connections server has sesrved
    failed_connections: u32,     //total failed connections

    last_request_time: SystemTime, //time of latest request
    last_health_check: SystemTime, //time of latest health check

    response_time: f64,     //last response time
    avg_response_time: f64, //avegage response time

    is_alive: bool, //is server alive?
}

impl Server {
    //creates and returns a new server
    pub fn new(uri: Uri) -> Self {
        Server {
            host: uri.host().unwrap().to_string(),
            port: uri.port_u16().unwrap(),
            uri: uri,

            max_connections: 100,
            connections: 0,
            total_connections: 0,
            successful_connections: 0,
            failed_connections: 0,

            last_request_time: SystemTime::now(),
            last_health_check: SystemTime::now(),

            response_time: 0.0,
            avg_response_time: 0.0,

            is_alive: true,
        }
    }

    //establishes connection with server and transfers data between server and client
    pub async fn transfer_data(
        server: SyncServer,
        client_stream: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //get host and port value from server
        let (host, port) = {
            let server_locked = server.lock().unwrap();
            (server_locked.host.clone(), server_locked.port)
        };

        //create a new server stream
        let server_stream = TcpStream::connect((host.as_str(), port)).await?;

        //split server and client streams into read and write streams
        let (mut client_read, mut client_write) = split(client_stream);
        let (mut server_read, mut server_write) = split(server_stream);

        //transfer data from client to server
        let client_to_server = tokio::spawn(async move {
            copy(&mut client_read, &mut server_write).await?;
            server_write.shutdown().await
        });

        //transfer data from server to client
        let server_to_server = tokio::spawn(async move {
            copy(&mut server_read, &mut client_write).await?;
            client_write.shutdown().await
        });

        //run both diretions concurrently
        let _ = try_join!(client_to_server, server_to_server)?;

        Ok(())
    }

    //handle_request handles incoming request and forwards it to a server
    //returns the response from the server
    pub async fn handle_request(
        server: SyncServer,
        mut req: Request<Incoming>,
        addr: SocketAddr,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        //get host, port and uri value from server
        let (host, port, uri) = {
            let server_locked = server.lock().unwrap();
            (
                server_locked.host.clone(),
                server_locked.port,
                server_locked.uri.clone(),
            )
        };

        //update the headers
        let headers = req.headers_mut();
        //update host in header
        let new_host_header = HeaderValue::from_str(host.as_str()).unwrap();
        headers.insert("host", new_host_header);
        //add FORWARDED to the headers
        headers.insert(
            FORWARDED,
            HeaderValue::from_str(
                format!(
                    "by={}; for={}; host={}; proto={}",
                    //by: load balancer address
                    "127.0.0.1:8000",
                    //for: client address
                    addr,
                    //host: server address
                    uri.to_string(),
                    //prototype: http1
                    "http1"
                )
                .as_str(),
            )
            .unwrap(),
        );

        //create a new stream to communicate with server
        let stream = TcpStream::connect((host.as_str(), port)).await.unwrap();
        let io = TokioIo::new(stream);

        //create an Hyper client
        let (mut sender, conn) = Builder::new()
            .preserve_header_case(true)
            .title_case_headers(true)
            .handshake(io)
            .await?;

        //spawn a task to poll the connection
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        //await the server response
        let resp = sender.send_request(req).await?;

        //convert Incoming into BoxBody and return the response
        Ok(resp.map(|b| b.boxed()))
    }
}
