use std::error::Error;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;

#[derive(Debug)]
struct NetworkStats {
    bytes_sent: usize,
    bytes_received: usize,
    elapsed_time: Duration,
    upload_rate: f64,   // bytes per second
    download_rate: f64, // bytes per second
    ratio: f64,         // download/upload ratio
}

async fn measure_transfer(
    address: &str,
    data_size: usize,
    iterations: usize,
) -> Result<NetworkStats, Box<dyn Error>> {
    let mut total_sent = 0;
    let mut total_received = 0;
    let start_time = Instant::now();

    for _ in 0..iterations {
        let mut stream = TcpStream::connect(address).await?;
        
        // Create some dummy data to send
        let data = vec![0u8; data_size];
        
        // Send data
        stream.write_all(&data).await?;
        total_sent += data.len();
        
        // Receive response
        let mut buffer = vec![0u8; 8192]; // 8KB buffer
        let n = stream.read(&mut buffer).await?;
        total_received += n;
        
        // Add a small delay between iterations
        sleep(Duration::from_millis(100)).await;
    }

    let elapsed = start_time.elapsed();
    let upload_rate = total_sent as f64 / elapsed.as_secs_f64();
    let download_rate = total_received as f64 / elapsed.as_secs_f64();
    let ratio = if total_sent > 0 {
        total_received as f64 / total_sent as f64
    } else {
        0.0
    };

    Ok(NetworkStats {
        bytes_sent: total_sent,
        bytes_received: total_received,
        elapsed_time: elapsed,
        upload_rate,
        download_rate,
        ratio,
    })
}

fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_rate(bytes_per_sec: f64) -> String {
    if bytes_per_sec < 1024.0 {
        format!("{:.2} B/s", bytes_per_sec)
    } else if bytes_per_sec < 1024.0 * 1024.0 {
        format!("{:.2} KB/s", bytes_per_sec / 1024.0)
    } else if bytes_per_sec < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.2} MB/s", bytes_per_sec / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB/s", bytes_per_sec / (1024.0 * 1024.0 * 1024.0))
    }
}

// Mock server for testing
// async fn run_mock_server() -> Result<(), Box<dyn Error>> {
async fn run_mock_server() -> Result<(), Box<dyn Error + Send + Sync>> {
 
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Mock server listening on127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = vec![0u8; 8192];
            match socket.read(&mut buf).await {
                Ok(n) => {
                    if n > 0 {
                        // Echo back a response that's half the size of what we received
                        let response = vec![1u8; n / 2]; //  responds with a vector of 1u8 bytes that is half the size of what was received
                        // 1u8 is the value (a byte with value 1), n / 2 is the number of times to repeat this value
                        let _ = socket.write_all(&response).await;
                    }
                }
                Err(e) => eprintln!("Socket read error: {}", e),
            }
        });
    }
}

// A more comprehensive monitoring example that could track a TLSNotary-like application
async fn monitor_mpc_simulation(data_sizes: &[usize]) -> Result<(), Box<dyn Error>> {
    println!("MPC Communication Simulation");
    println!("============================\n");
    
    for &size in data_sizes {
        println!("Testing with {} payload", format_bytes(size));
        
        // Simulate MPC overhead based on TLSNotary's documentation
        let base_overhead = 25 * 1024 * 1024; // 25MB fixed cost
        let upload_overhead_factor = 10.0; // 10MB per 1KB of outgoing data
        let download_overhead_factor = 0.04; // 40KB per 1KB of incoming data
        
        let upload_total = base_overhead + (size as f64 * upload_overhead_factor) as usize;
        let response_size = size * 10; // Assume response is 10x the request size
        let download_total = (response_size as f64 * download_overhead_factor) as usize;
        
        println!("  Simulated request size: {}", format_bytes(size));
        println!("  Simulated response size: {}", format_bytes(response_size));
        println!("  Estimated MPC upload overhead: {}", format_bytes(upload_total));
        println!("  Estimated MPC download overhead: {}", format_bytes(download_total));
        println!("  Overhead ratio: {:.2}x upload, {:.2}x download", 
                upload_total as f64 / size as f64,
                download_total as f64 / response_size as f64);
        println!();
        
        // Could add actual network transfer test here with mock MPC
    }
    
    Ok(())
}

// #[tokio::main]
pub async fn run_main() -> Result<(), Box<dyn Error>> {
    // Start mock server in the background
    tokio::spawn(run_mock_server());
    
    // Wait for the server to start
    sleep(Duration::from_millis(500)).await;
    
    // First, measure some actual transfers
    println!("Network Transfer Test");
    println!("====================\n");
    
    let test_sizes = [1024, 10 * 1024, 100 * 1024];
    for size in test_sizes {
        println!("Testing with {} payload", format_bytes(size));
        
        match measure_transfer("127.0.0.1:8080", size, 5).await {
            Ok(stats) => {
                println!("  Sent: {}", format_bytes(stats.bytes_sent));
                println!("  Received: {}", format_bytes(stats.bytes_received));
                println!("  Time: {:.2?}", stats.elapsed_time);
                println!("  Upload: {}", format_rate(stats.upload_rate));
                println!("  Download: {}", format_rate(stats.download_rate));
                println!("  Ratio (received/sent): {:.2}", stats.ratio);
                println!();
            }
            Err(e) => {
                eprintln!("Error measuring transfer: {}", e);
            }
        }
    }
    
    // Then simulate MPC overhead calculations
    monitor_mpc_simulation(&[1024, 10 * 1024]).await?;
    
    Ok(())
}

// To extend this exercise:
// 1. Add real TLS communication with reqwest or hyper crates
// 2. Implement a proxy to measure traffic between two endpoints
// 3. Add visualization of bandwidth usage over time
// 4. Create a CLI interface to configure test parameters
// 5. Add support for measuring existing applications' network traffic