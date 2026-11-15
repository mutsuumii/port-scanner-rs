# port-scanner-rs 🦀

Fast asynchronous TCP port scanner written in Rust.

## Features
- Async I/O with Tokio — scans thousands of ports in seconds
- Configurable concurrency and timeout
- DNS resolution support

## Build & Run
```bash
cargo build --release
./target/release/port-scanner-rs 192.168.1.1 -s 1 -e 65535 -c 1000
```

## Usage
```
port-scanner-rs <TARGET> [-s START] [-e END] [-c CONCURRENCY] [-t TIMEOUT_MS]
```

## License
MIT
