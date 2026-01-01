use deston::config::config::Config;
use deston::load_balancer::layer4::Layer4;
use deston::load_balancer::layer7::Layer7;
use deston::load_balancer::load_balancer::LoadBalancer;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_layer4_graceful_shutdown() {
    // Create a test config
    let config_content = r#"
[load_balancer]
address = "127.0.0.1"
port = 18080
algorithm = "round_robin"
layer = "L4"

[[server]]
address = "127.0.0.1"
port = 13000
max_connections = 1000
weight = 1
"#;

    let mut config_path = std::env::temp_dir();
    config_path.push("test_shutdown_l4.toml");
    fs::write(&config_path, config_content).unwrap();

    let config = Config::new(&config_path);
    let config_arc = Arc::new(Mutex::new(config));

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    let lb = Layer4::new(config_arc);

    // Start the load balancer in a separate task
    let lb_handle = tokio::spawn(async move { lb.start(shutdown_rx).await });

    // Give it a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send shutdown signal
    let _ = shutdown_tx.send(true);

    // The load balancer should shut down gracefully
    let result = timeout(Duration::from_secs(5), lb_handle).await;

    // Clean up
    fs::remove_file(config_path).ok();

    // Assert that the load balancer shut down successfully
    assert!(
        result.is_ok(),
        "Load balancer should shut down within timeout"
    );
    assert!(
        result.unwrap().is_ok(),
        "Load balancer should return Ok on shutdown"
    );
}

#[tokio::test]
async fn test_layer7_graceful_shutdown() {
    // Create a test config
    let config_content = r#"
[load_balancer]
address = "127.0.0.1"
port = 18081
algorithm = "round_robin"
layer = "L7"

[[server]]
address = "127.0.0.1"
port = 13001
max_connections = 1000
weight = 1
"#;

    let mut config_path = std::env::temp_dir();
    config_path.push("test_shutdown_l7.toml");
    fs::write(&config_path, config_content).unwrap();

    let config = Config::new(&config_path);
    let config_arc = Arc::new(Mutex::new(config));

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    let lb = Layer7::new(config_arc);

    // Start the load balancer in a separate task
    let lb_handle = tokio::spawn(async move { lb.start(shutdown_rx).await });

    // Give it a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send shutdown signal
    let _ = shutdown_tx.send(true);

    // The load balancer should shut down gracefully
    let result = timeout(Duration::from_secs(5), lb_handle).await;

    // Clean up
    fs::remove_file(config_path).ok();

    // Assert that the load balancer shut down successfully
    assert!(
        result.is_ok(),
        "Load balancer should shut down within timeout"
    );
    assert!(
        result.unwrap().is_ok(),
        "Load balancer should return Ok on shutdown"
    );
}

#[tokio::test]
async fn test_shutdown_without_signal() {
    // Test that shutdown channel works correctly when not triggered
    let config_content = r#"
[load_balancer]
address = "127.0.0.1"
port = 18082
algorithm = "round_robin"
layer = "L7"

[[server]]
address = "127.0.0.1"
port = 13002
max_connections = 1000
weight = 1
"#;

    let mut config_path = std::env::temp_dir();
    config_path.push("test_shutdown_no_signal.toml");
    fs::write(&config_path, config_content).unwrap();

    let config = Config::new(&config_path);
    let config_arc = Arc::new(Mutex::new(config));

    // Create shutdown channel
    let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    let lb = Layer7::new(config_arc);

    // Start the load balancer in a separate task
    let lb_handle = tokio::spawn(async move { lb.start(shutdown_rx).await });

    // Give it a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Without sending a shutdown signal, the task should still be running
    // We use a very short timeout to verify it's still running
    let result = timeout(Duration::from_millis(200), lb_handle).await;

    // Clean up
    fs::remove_file(config_path).ok();

    // Assert that the load balancer is still running (timeout occurs)
    assert!(
        result.is_err(),
        "Load balancer should still be running without shutdown signal"
    );
}
