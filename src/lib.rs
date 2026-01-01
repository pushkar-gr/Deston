// Library exports for Deston load balancer
//
//! # Deston Load Balancer
//!
//! A high-performance Layer 4 (L4) and Layer 7 (L7) load balancer implementation.
//!
//! ## Modules
//!
//! - `config`: Configuration parsing and management
//! - `load_balancer`: Load balancer trait and implementations (Layer 4 and Layer 7)
//! - `server`: Backend server management and request handling
//!
//! ## Example
//!
//! ```no_run
//! use Deston::config::config::Config;
//! use Deston::load_balancer::load_balancer::LoadBalancer;
//! use Deston::load_balancer::layer4::Layer4;
//! use std::path::Path;
//! use std::sync::{Arc, Mutex};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = Config::new(Path::new("config.toml"));
//!     let lb = Layer4::new(Arc::new(Mutex::new(config)));
//!     let _ = lb.start().await;
//! }
//! ```

pub mod config;
pub mod load_balancer;
pub mod server;

// Re-export Arc for convenience
pub use std::sync::Arc;
