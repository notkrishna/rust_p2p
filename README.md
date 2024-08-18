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
3. Change the config.toml if you want to. Default configuration runs on 127.0.0.1:8080.
```toml
#Example cofig file
[broadcast]
address = "127.0.0.1:8080"

[peers]
addresses = ["127.0.0.1:8081", "127.0.0.1:8082"]
```


4. Run one primary instance (brodcaster)
```bash
cargo run
```
5. Run peers on different nodes. Or clone the repository and change the config.toml to create another node on the same machine.
```toml
[broadcast]
address = "127.0.0.1:8081"

[peers]
addresses = ["127.0.0.1:8080", "127.0.0.1:8082"]

```

```toml
[broadcast]
address = "127.0.0.1:8082"

[peers]
addresses = ["127.0.0.1:8080", "127.0.0.1:8081"]

```

Now you can start writing messages.