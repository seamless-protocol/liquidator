# Aave V3 Liquidator

This is an [Artemis](https://github.com/paradigmxyz/artemis) bot that liquidates aave v3 positions on base chain

# Getting Started

1. Install Rust [ref](https://doc.rust-lang.org/book/ch01-01-installation.html)
    - Install deps: `sudo apt install -y build-essential libssl-dev` (Linux) or `xcode-select --install` (Mac)

3. Install Foundry [ref](https://book.getfoundry.sh/getting-started/installation)

3. Deploy liquidator contract
```bash
cd crates/liquidator-contract
forge install
forge create ./src/Liquidator.sol/Liquidator --private-key <xyz> --rpc-url <xyz>
```

4. Build Rust Application

a. cd to root of project

b. (if not cross compiling) Compile binary for host arch: 
```bash
cargo build --release
```

c. (if cross compiling) Compile binary on Mac for Linux: 
Cross is a tool developed by Rust devs to simplify cross compilation (compiling on one machine architecture for another architecture, e.g.: compile binary on mac for running on a linux machine). Make sure to install from github repo not latest release in cargo (it's quite outdated). [Github link](https://github.com/cross-rs/cross)

If you are using an ARM based Mac (e.g.: M1), you need to set DOCKER_DEFAULT_PLATFORM env to `linux/amd64`

```bash
cargo install cross --git https://github.com/cross-rs/cross
DOCKER_DEFAULT_PLATFORM=linux/amd64 cross build --target x86_64-unknown-linux-gnu --release
```

d. Copy binary to liquidator host
`scp ./target/x86_64-unknown-linux-gnu/release/aave-v3-liquidator liquidator:/home/0xforestwalker/repos/artemis-aave-v3-liquidator/aave-v3-liquidator`

6. Run liquidator
`./target/release/aave-v3-liquidator --rpc <xyz> --private-key <xyz> --bid-percentage 100 --deployent seashell --liquidator-address <xyz>`

STIG Liquidator addres: `0xB2e66FeEe21DEB1e179a446BAb0B311412E592Db`