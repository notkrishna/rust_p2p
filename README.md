# Rust P2P
rust_p2p is a simple peer-to-peer (P2P) network implemented in Rust using the Tokio library for asynchronous networking. The application allows multiple nodes to discover each other, communicate, and exchange messages over a network.

### > 🚧 **Experimental:** This feature is experimental and may change.


## Dependencies
Make sure you have Rust and cargo installed. Install them from https://www.rust-lang.org/tools/install.

## Usage
1. Clone the repo
```bash
git clone https://github.com/notkrishna/rust_p2p.git
cd rust_p2p
```
2. Build the project
```bash
cargo build
```
3. Run one primary instance (brodcaster)
```bash
cargo run -- 127.0.0.1:8080
```
4. Run two secondary instances (receivers) in seperate terminals.
> **Note:** The IPs should be same as mentioned below.
```bash
cargo run -- 127.0.0.1:8081
```

```bash
cargo run -- 127.0.0.1:8082
```

Now you can start writing messages in broadcaster's terminal.