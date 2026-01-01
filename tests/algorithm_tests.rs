use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

// Import algorithm modules from main crate
use deston::load_balancer::algorithm::algorithm::Algorithm;
use deston::load_balancer::algorithm::r#static::{
    ip_hashing::IpHashing, round_robin::RoundRobin, weighted_round_robin::WeightedRoundRobin,
};
use deston::server::server::Server;
use hyper::Uri;

// Helper function to create test servers
fn create_test_servers(count: usize, weights: Option<Vec<usize>>) -> Arc<Vec<Arc<Mutex<Server>>>> {
    let servers: Vec<Arc<Mutex<Server>>> = (0..count)
        .map(|i| {
            let weight = weights.as_ref().map(|w| w[i]).unwrap_or(1);
            let port = 3000 + i;
            let uri = format!("http://127.0.0.1:{}", port).parse::<Uri>().unwrap();
            Arc::new(Mutex::new(Server::new(uri, 1000, weight)))
        })
        .collect();
    Arc::new(servers)
}

// Helper to create a test socket address
fn test_addr() -> SocketAddr {
    "127.0.0.1:5000".parse().unwrap()
}

#[test]
fn test_round_robin_basic() {
    let mut algorithm = RoundRobin::new();
    let servers = create_test_servers(3, None);

    // Pick servers in round-robin order
    let (index1, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();
    let (index2, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();
    let (index3, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();
    let (index4, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();

    assert_eq!(index1, 0);
    assert_eq!(index2, 1);
    assert_eq!(index3, 2);
    assert_eq!(index4, 0); // Wraps around
}

#[test]
fn test_round_robin_single_server() {
    let mut algorithm = RoundRobin::new();
    let servers = create_test_servers(1, None);

    // Should always return the same server
    let (index1, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();
    let (index2, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();

    assert_eq!(index1, 0);
    assert_eq!(index2, 0);
}

#[test]
fn test_weighted_round_robin_equal_weights() {
    let mut algorithm = WeightedRoundRobin::new();
    let servers = create_test_servers(2, Some(vec![1, 1]));

    // With equal weights, should alternate
    let (index1, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();
    let (index2, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();
    let (index3, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();

    assert_eq!(index1, 0);
    assert_eq!(index2, 1);
    assert_eq!(index3, 0);
}

#[test]
fn test_weighted_round_robin_different_weights() {
    let mut algorithm = WeightedRoundRobin::new();
    let servers = create_test_servers(2, Some(vec![3, 1]));

    // Server 0 with weight 3 should be picked 3 times before server 1
    let mut picks = vec![];
    for _ in 0..8 {
        let (index, _) = algorithm.pick_server(servers.clone(), test_addr()).unwrap();
        picks.push(index);
    }

    // First 4 picks should follow pattern: 0, 0, 0, 1
    assert_eq!(picks[0], 0);
    assert_eq!(picks[1], 0);
    assert_eq!(picks[2], 0);
    assert_eq!(picks[3], 1);
    // Pattern repeats
    assert_eq!(picks[4], 0);
    assert_eq!(picks[5], 0);
    assert_eq!(picks[6], 0);
    assert_eq!(picks[7], 1);
}

#[test]
fn test_ip_hashing_consistency() {
    let mut algorithm = IpHashing::new();
    let servers = create_test_servers(3, None);

    let addr1: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    let addr2: SocketAddr = "127.0.0.1:5001".parse().unwrap();

    // Same IP should always map to same server
    let (index1, _) = algorithm.pick_server(servers.clone(), addr1).unwrap();
    let (index2, _) = algorithm.pick_server(servers.clone(), addr1).unwrap();
    assert_eq!(index1, index2);

    // Different IPs might map to different servers
    let (index3, _) = algorithm.pick_server(servers.clone(), addr2).unwrap();

    // All indices should be valid
    assert!(index1 < 3);
    assert!(index3 < 3);
}

#[test]
fn test_ip_hashing_distribution() {
    let mut algorithm = IpHashing::new();
    let servers = create_test_servers(3, None);

    // Test multiple different IPs
    let addrs: Vec<SocketAddr> = (5000..5010)
        .map(|port| format!("127.0.0.1:{}", port).parse().unwrap())
        .collect();

    let mut indices = vec![];
    for addr in addrs {
        let (index, _) = algorithm.pick_server(servers.clone(), addr).unwrap();
        indices.push(index);
    }

    // Should have distribution across servers (not all same index)
    let unique_indices: std::collections::HashSet<_> = indices.iter().collect();
    assert!(
        unique_indices.len() > 1,
        "IP hashing should distribute across multiple servers"
    );
}

#[test]
fn test_server_creation() {
    let uri = "http://127.0.0.1:3000".parse::<Uri>().unwrap();
    let server = Server::new(uri, 1000, 5);

    assert_eq!(server.weight, 5);
}
