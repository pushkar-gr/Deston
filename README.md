# Deston

> **A High-Performance, Rust-based Layer 4 (TCP) & Layer 7 (HTTP) Load Balancer.**

[![Build Status](https://img.shields.io/github/actions/workflow/status/pushkar-gr/Deston/build.yml?branch=main)](https://github.com/pushkar-gr/Deston/actions)
[![License](https://img.shields.io/github/license/pushkar-gr/Deston)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org)

Deston is a lightweight, asynchronous load balancer built in **Rust**. It is designed for speed and flexibility, offering dual support for transport-layer (L4) and application-layer (L7) load balancing. Built on top of `tokio` and `hyper`, Deston handles high-concurrency traffic with ease.

---

## 🏗️ Architecture

Deston sits between clients and your backend infrastructure, intelligently distributing traffic based on your configuration. It supports both raw TCP streams (L4) for maximum throughput and HTTP (L7) for header-aware routing.

[![](https://mermaid.ink/img/pako:eNqNVF2PojAU_StNn9UA4qA8bDI62YwJkxhwY7JoNhWu0Ay0bimz66r_fS-gI7Kzm-1D6T3n3I-W2x5pJGOgLk0U26dk-bQWBMeXAlQ4yzgITXz4XkKhNw0zk2LHkzCqPwMt8-xCNHNRbptIT-giBfEki8mUZUxEoBpJNV4YF2GO00AVmxvsTb8tFeP6eOdGloc9nG-q2-o9W6AV05BwKG5kNR6zRIbVpLhOc1IH39xLfD_0ZSli4sstFx1yhewKeJJqQIHfYeeLZ1ak4XxBqi8XSYsHEf-zYo8dQHWq9eywholNlrMF8XihQYDqZPWci8ohz8vlh7K_JA9AvYFaSJmFzbL2JUHKFG4v0HiElyDvATp_dcqiV-RadQfmNZZJ3KFhGK0yAuvKWTVntqO324n0-5_qnmjAalVD135o4KtVUadZqRR2Z3Ygj5Hmb3DC0_tANs_3GeQoxA1uS03mgl3lTnuHVcM3Ke0ucKfz7BqsWuoCOB2gmSu7LiCADCJdnFqH31YM0Nf3u8jqT6jptXaKW8A60WepfjAVF1WT73Y8wozm_wot2sMHgMfU1aqEHs1B4eVEkx6rEGuqUzzENXVxGTP1uqZrcUafPRNfpcyvbkqWSUrdHcsKtMp9jA31xBm2zk2C_x7UDC-cpq41qUNQ90h_UtccOwPTNCxrYk8sNEY9eqBu3xoMx5OhZZjDkWk-jBzn3KO_6qTGYGLbtvEwHo6NiTUyHKdHIeZaqpfmTWseKXr-DQOEb4A?type=png)](https://mermaid.live/edit#pako:eNqNVF2PojAU_StNn9UA4qA8bDI62YwJkxhwY7JoNhWu0Ay0bimz66r_fS-gI7Kzm-1D6T3n3I-W2x5pJGOgLk0U26dk-bQWBMeXAlQ4yzgITXz4XkKhNw0zk2LHkzCqPwMt8-xCNHNRbptIT-giBfEki8mUZUxEoBpJNV4YF2GO00AVmxvsTb8tFeP6eOdGloc9nG-q2-o9W6AV05BwKG5kNR6zRIbVpLhOc1IH39xLfD_0ZSli4sstFx1yhewKeJJqQIHfYeeLZ1ak4XxBqi8XSYsHEf-zYo8dQHWq9eywholNlrMF8XihQYDqZPWci8ohz8vlh7K_JA9AvYFaSJmFzbL2JUHKFG4v0HiElyDvATp_dcqiV-RadQfmNZZJ3KFhGK0yAuvKWTVntqO324n0-5_qnmjAalVD135o4KtVUadZqRR2Z3Ygj5Hmb3DC0_tANs_3GeQoxA1uS03mgl3lTnuHVcM3Ke0ucKfz7BqsWuoCOB2gmSu7LiCADCJdnFqH31YM0Nf3u8jqT6jptXaKW8A60WepfjAVF1WT73Y8wozm_wot2sMHgMfU1aqEHs1B4eVEkx6rEGuqUzzENXVxGTP1uqZrcUafPRNfpcyvbkqWSUrdHcsKtMp9jA31xBm2zk2C_x7UDC-cpq41qUNQ90h_UtccOwPTNCxrYk8sNEY9eqBu3xoMx5OhZZjDkWk-jBzn3KO_6qTGYGLbtvEwHo6NiTUyHKdHIeZaqpfmTWseKXr-DQOEb4A)

---

## ✨ Key Features

- **Dual-Mode Operation**: Switch between Layer 4 (TCP) and Layer 7 (HTTP) modes via a simple config change.
- **Asynchronous & Non-Blocking**: Built on `tokio` to handle thousands of concurrent connections efficiently.
- **Header Injection (L7)**: Automatically injects `X-Forwarded-For` and `Host` headers for backend transparency.
- **Pluggable Algorithms**: Choose the distribution strategy that best fits your traffic patterns.
- **Session Affinity**: Built-in support for IP Hashing to ensure clients stick to specific servers.

---

## 🧠 Load Balancing Algorithms

Deston supports three core algorithms, configurable in `config.toml`:

| Algorithm | Key | Description |
|-----------|-----|-------------|
| **Round Robin** | `round_robin` | Distributes requests sequentially across all available servers. Ideal for stateless backends with equal capacity. |
| **Weighted Round Robin** | `weighted_round_robin` | Respects the `weight` parameter. Servers with higher weights receive proportionally more traffic. Perfect for heterogeneous server clusters. |
| **IP Hashing** | `ip_hashing` | Uses the client's IP address to determine the server. Ensures the same client always reaches the same server (Session Persistence). |

---

## 🚀 Quick Start

### Prerequisites
- **Rust**: Version 1.70 or higher.
- **Cargo**: Included with Rust installation.

### Installation

1. **Clone the repository:**
```bash
git clone https://github.com/pushkar-gr/Deston.git
cd Deston

```

2. **Build the project:**
```bash
cargo build --release

```


3. **Run the Load Balancer:**
```bash
cargo run --release

```



---

## ⚙️ Configuration

Deston is configured via a `config.toml` file in the root directory.

### Example Configuration

```toml
[load_balancer]
address = "127.0.0.1"   # The IP Deston will bind to
port = 8080             # The port Deston will listen on
algorithm = "round_robin" # Options: round_robin, weighted_round_robin, ip_hashing
layer = "L7"            # Options: L4, L7

# Backend Server 1
[[server]]
address = "127.0.0.1"
port = 3000
max_connections = 1000
weight = 3              # Higher weight = more traffic (if using weighted_round_robin)

# Backend Server 2
[[server]]
address = "127.0.0.1"
port = 3001
max_connections = 1000
weight = 1

```

### Configuration Parameters

**[load_balancer]**

* **layer**: `L4` (TCP) or `L7` (HTTP). Defaults to `L4` if unspecified.
* **algorithm**: The strategy for picking servers. Case-insensitive (e.g., `RoundRobin`, `ip_hashing`).
* **address**: The host address to bind (e.g., `0.0.0.0` for public access).
* **port**: The listening port.

**[[server]]**

* **address/port**: The location of the backend instance.
* **max_connections**: Hard limit on concurrent connections forwarded to this server.
* **weight**: Used only by `weighted_round_robin` to bias traffic distribution.

---

## 📂 Project Structure

* **`src/config`**: TOML parsing and configuration management.
* **`src/load_balancer`**:
* `layer4.rs`: Raw TCP stream forwarding implementation.
* `layer7.rs`: HTTP request parsing and forwarding via `hyper`.
* `algorithm/`: Implementation of routing logic (Static, Hashing, etc.).


* **`src/server`**: Backend server connection handling and metric tracking.

---

## 🤝 Contributing

Contributions are welcome!

1. Fork the project.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.
