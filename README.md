# Network Traffic Monitoring

A Rust-based utility for measuring and analyzing network traffic performance, with a focus on simulating and monitoring Multi-Party Computation (MPC) communication patterns.

## Features

- Measure actual network performance between endpoints
- Calculate upload and download rates
- Simulate MPC protocol overhead
- Include a mock server for local testing
- Format data sizes in human-readable form (B, KB, MB, GB)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56.0 or later)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (included with Rust)

## Project Structure

```
Network_Traffic_Monitoring/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── src/
    ├── main.rs     # Main entry point
    └── tool.rs     # Network monitoring implementation
```

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd Network_Traffic_Monitoring
   ```

2. Check that your Cargo.toml has the necessary dependencies:
   ```toml
   [package]
   name = "network_traffic_monitoring"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   tokio = { version = "1", features = ["full"] }
   ```

## Usage

### Running the Application

```bash
cargo run
```

This will:
1. Start a mock server on localhost (127.0.0.1:8080)
2. Run network transfer tests with various payload sizes
3. Display statistics for each test
4. Simulate MPC communication overhead

### Example Output

```
Network Transfer Test
====================

Testing with 1.00 KB payload
  Sent: 1.00 KB
  Received: 512.00 B
  Time: 538.21ms
  Upload: 1.86 KB/s
  Download: 952.38 B/s
  Ratio (received/sent): 0.50

Testing with 10.00 KB payload
  Sent: 50.00 KB
  Received: 25.00 KB
  Time: 564.89ms
  Upload: 88.51 KB/s
  Download: 44.26 KB/s
  Ratio (received/sent): 0.50

...

MPC Communication Simulation
============================

Testing with 1.00 KB payload
  Simulated request size: 1.00 KB
  Simulated response size: 10.00 KB
  Estimated MPC upload overhead: 25.01 MB
  Estimated MPC download overhead: 400.00 B
  Overhead ratio: 25619.97x upload, 0.04x download
```

## Code Overview

### Main Components

1. **NetworkStats (struct)**: Stores metrics about network transfers
   - bytes_sent, bytes_received
   - elapsed_time
   - upload_rate, download_rate
   - ratio (download/upload)

2. **measure_transfer (async function)**: Performs actual network transfers and collects statistics
   - Connects to a specified address
   - Sends data of a given size
   - Receives response
   - Calculates performance metrics

3. **run_mock_server (async function)**: Local TCP server for testing
   - Listens on 127.0.0.1:8080
   - Echoes back half the size of received data

4. **monitor_mpc_simulation (async function)**: Simulates MPC protocol overhead
   - Calculates estimated data transfer for MPC protocols
   - Based on TLSNotary-like parameters

5. **Utilities**:
   - format_bytes: Converts byte counts to human-readable format
   - format_rate: Formats data rates with appropriate units

## Extending the Project

As mentioned in the code comments, you could extend this project by:

1. Adding real TLS communication with reqwest or hyper crates
2. Implementing a proxy to measure traffic between two endpoints
3. Adding visualization of bandwidth usage over time
4. Creating a CLI interface to configure test parameters
5. Adding support for measuring existing applications' network traffic

## Troubleshooting

### Common Issues

1. **Error: address already in use**
   - The mock server is already running or another service is using port 8080
   - Solution: Change the port in `run_mock_server` function

2. **Cargo build errors**
   - Ensure you have the latest version of dependencies
   - Run `cargo update` to update dependencies

3. **Performance discrepancies**
   - Network performance can vary based on system load
   - For consistent results, run tests multiple times and average the results

## License

[MIT License](LICENSE)