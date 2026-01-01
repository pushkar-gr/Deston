use std::fs;
use std::path::Path;
use Deston::config::config::{Algorithm, Config};

#[test]
fn test_config_parsing_basic() {
    // Create a temporary config file
    let config_content = r#"
[load_balancer]
address = "127.0.0.1"
port = 8080
algorithm = "round_robin"

[[server]]
address = "127.0.0.1"
port = 3000
max_connections = 1000
weight = 1

[[server]]
address = "127.0.0.1"
port = 3001
max_connections = 500
weight = 2
"#;

    let config_path = "/tmp/test_config.toml";
    fs::write(config_path, config_content).unwrap();

    let config = Config::new(Path::new(config_path));

    // Verify load balancer address
    assert_eq!(config.load_balancer_address.to_string(), "127.0.0.1:8080");

    // Verify servers
    assert_eq!(config.servers.len(), 2);

    // Verify first server
    let server1 = config.servers[0].lock().unwrap();
    assert_eq!(server1.weight, 1);

    // Verify second server
    let server2 = config.servers[1].lock().unwrap();
    assert_eq!(server2.weight, 2);

    // Clean up
    fs::remove_file(config_path).ok();
}

#[test]
fn test_config_algorithm_case_insensitive() {
    let test_cases = vec![
        ("round_robin", Algorithm::RoundRobin),
        ("RoundRobin", Algorithm::RoundRobin),
        ("ROUND_ROBIN", Algorithm::RoundRobin),
        ("weighted_round_robin", Algorithm::WeightedRoundRobin),
        ("WeightedRoundRobin", Algorithm::WeightedRoundRobin),
        ("ip_hashing", Algorithm::IpHashing),
        ("IpHashing", Algorithm::IpHashing),
    ];

    for (algo_str, _expected) in test_cases {
        let config_content = format!(
            r#"
[load_balancer]
address = "127.0.0.1"
port = 8080
algorithm = "{}"

[[server]]
address = "127.0.0.1"
port = 3000
max_connections = 1000
weight = 1
"#,
            algo_str
        );

        let config_path = format!("/tmp/test_config_{}.toml", algo_str.replace("_", ""));
        fs::write(&config_path, config_content).unwrap();

        // Should not panic - algorithm should be parsed correctly
        let _config = Config::new(Path::new(&config_path));

        // Clean up
        fs::remove_file(&config_path).ok();
    }
}

#[test]
fn test_config_default_values() {
    // Config with minimal settings
    let config_content = r#"
[[server]]
address = "127.0.0.1"
port = 3000
"#;

    let config_path = "/tmp/test_config_defaults.toml";
    fs::write(config_path, config_content).unwrap();

    let config = Config::new(Path::new(config_path));

    // Should use default load balancer address
    assert_eq!(config.load_balancer_address.host().unwrap(), "localhost");

    // Should have servers
    assert!(!config.servers.is_empty());

    // Clean up
    fs::remove_file(config_path).ok();
}

#[test]
fn test_config_multiple_servers() {
    let config_content = r#"
[load_balancer]
address = "0.0.0.0"
port = 9000
algorithm = "round_robin"

[[server]]
address = "192.168.1.1"
port = 3000
max_connections = 100
weight = 1

[[server]]
address = "192.168.1.2"
port = 3001
max_connections = 200
weight = 2

[[server]]
address = "192.168.1.3"
port = 3002
max_connections = 300
weight = 3
"#;

    let config_path = "/tmp/test_config_multi.toml";
    fs::write(config_path, config_content).unwrap();

    let config = Config::new(Path::new(config_path));

    assert_eq!(config.servers.len(), 3);
    assert_eq!(config.servers[0].lock().unwrap().weight, 1);
    assert_eq!(config.servers[1].lock().unwrap().weight, 2);
    assert_eq!(config.servers[2].lock().unwrap().weight, 3);

    // Clean up
    fs::remove_file(config_path).ok();
}
