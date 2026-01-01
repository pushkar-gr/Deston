use deston::config::config::{Config, LayerMode};
use std::fs;
use std::path::Path;

#[test]
fn test_l4_config_file() {
    // Test with the default config.toml that has L4 mode
    let config_content = r#"
[load_balancer]
address = "127.0.0.1"
port = 8080
algorithm = "round_robin"
layer = "L4"

[[server]]
address = "127.0.0.1"
port = 3000
max_connections = 1000
weight = 1

[[server]]
address = "127.0.0.1"
port = 3001
max_connections = 1000
weight = 1
"#;

    let config_path = "/tmp/test_integration_l4.toml";
    fs::write(config_path, config_content).unwrap();

    let config = Config::new(Path::new(config_path));

    // Verify L4 configuration
    assert_eq!(config.layer_mode, LayerMode::L4);
    assert_eq!(config.load_balancer_address.to_string(), "127.0.0.1:8080");
    assert_eq!(config.servers.len(), 2);

    // Clean up
    fs::remove_file(config_path).ok();
}

#[test]
fn test_l7_config_file() {
    // Test with L7 mode
    let config_content = r#"
[load_balancer]
address = "127.0.0.1"
port = 8080
algorithm = "round_robin"
layer = "L7"

[[server]]
address = "127.0.0.1"
port = 3000
max_connections = 1000
weight = 1

[[server]]
address = "127.0.0.1"
port = 3001
max_connections = 1000
weight = 1
"#;

    let config_path = "/tmp/test_integration_l7.toml";
    fs::write(config_path, config_content).unwrap();

    let config = Config::new(Path::new(config_path));

    // Verify L7 configuration
    assert_eq!(config.layer_mode, LayerMode::L7);
    assert_eq!(config.load_balancer_address.to_string(), "127.0.0.1:8080");
    assert_eq!(config.servers.len(), 2);

    // Clean up
    fs::remove_file(config_path).ok();
}

#[test]
fn test_backward_compatibility_no_layer() {
    // Test that missing layer defaults to L4 for backward compatibility
    let config_content = r#"
[load_balancer]
address = "127.0.0.1"
port = 8080
algorithm = "round_robin"

[[server]]
address = "127.0.0.1"
port = 3000
"#;

    let config_path = "/tmp/test_integration_backward_compat.toml";
    fs::write(config_path, config_content).unwrap();

    let config = Config::new(Path::new(config_path));

    // Should default to L4
    assert_eq!(config.layer_mode, LayerMode::L4);

    // Clean up
    fs::remove_file(config_path).ok();
}
