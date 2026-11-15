use clap::Parser;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "port-scanner-rs", about = "Fast async TCP port scanner")]
struct Args {
    /// Target IP or hostname
    target: String,

    /// Start port
    #[arg(short = 's', long, default_value_t = 1)]
    start: u16,

    /// End port
    #[arg(short = 'e', long, default_value_t = 1024)]
    end: u16,

    /// Max concurrent connections
    #[arg(short = 'c', long, default_value_t = 500)]
    concurrency: usize,

    /// Timeout in milliseconds
    #[arg(short = 't', long, default_value_t = 1000)]
    timeout_ms: u64,
}

async fn scan_port(target: &str, port: u16, timeout_ms: u64) -> Option<u16> {
    let addr = format!("{}:{}", target, port);
    if let Ok(sockaddr) = addr.parse::<SocketAddr>() {
        let dur = Duration::from_millis(timeout_ms);
        if timeout(dur, TcpStream::connect(sockaddr)).await.is_ok() {
            return Some(port);
        }
    } else {
        // Try DNS resolution
        if let Ok(addrs) = tokio::net::lookup_host(&addr).await {
            for sockaddr in addrs {
                let dur = Duration::from_millis(timeout_ms);
                if timeout(dur, TcpStream::connect(sockaddr)).await.is_ok() {
                    return Some(port);
                }
                break;
            }
        }
    }
    None
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let sem = Arc::new(Semaphore::new(args.concurrency));

    println!("[*] Scanning {} ports {}-{}", args.target, args.start, args.end);
    println!("[*] Concurrency: {}, Timeout: {}ms", args.concurrency, args.timeout_ms);

    let mut handles = Vec::new();
    let mut open_ports = Vec::new();

    for port in args.start..=args.end {
        let target = args.target.clone();
        let sem = Arc::clone(&sem);
        let timeout_ms = args.timeout_ms;

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            scan_port(&target, port, timeout_ms).await
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Ok(Some(port)) = handle.await {
            println!("  [+] {}/tcp OPEN", port);
            open_ports.push(port);
        }
    }

    open_ports.sort();
    println!("\n[*] Scan complete. {} open port(s) found.", open_ports.len());
    if !open_ports.is_empty() {
        let ports_str: Vec<String> = open_ports.iter().map(|p| p.to_string()).collect();
        println!("[*] Open: {}", ports_str.join(", "));
    }
}
