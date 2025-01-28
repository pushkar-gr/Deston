use http::header::{HeaderValue, FORWARDED};
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::body::{Bytes, Incoming};
use hyper::client::conn::http1::Builder;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Uri};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

//start_layer7 starts layer 7 load balancer
//will listen to incoming requests at given address and calls handle_request to forward request to a server
#[tokio::main]
pub async fn start_layer7() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //load balancer address
    let lb_address = "http://127.0.0.1:8000".parse::<Uri>().unwrap();
    let host = lb_address.host().unwrap();
    let port = lb_address.port_u16().unwrap();

    //create a TcpListener and binds it to load balancer address
    let listener = TcpListener::bind((host, port)).await?;

    //loop to continuously accetp incoming connections
    loop {
        let (stream, addr) = listener.accept().await?;
        let io = TokioIo::new(stream);

        //spawn a tokio task to server multiple connections concurrently
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                //bind the incoming connection to handle_request
                .serve_connection(io, service_fn(move |req| handle_request(req, addr)))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

//handle_request handles incoming request and forwards it to a server
//picks a server based on algorithm
//returns the response from the server
async fn handle_request(
    mut req: Request<Incoming>,
    addr: SocketAddr,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    //server address
    //!todo: pick the server based on an algorithm
    let server_uri = "http://127.0.0.1:3000".parse::<Uri>().unwrap();
    let host = server_uri.host().unwrap();
    let port = server_uri.port_u16().unwrap();

    //update the headers
    let headers = req.headers_mut();
    //update host in header
    let new_host_header = HeaderValue::from_str(host).unwrap();
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
                server_uri.to_string(),
                //proto
                "http1"
            )
            .as_str(),
        )
        .unwrap(),
    );

    let stream = TcpStream::connect((host, port)).await.unwrap();
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

