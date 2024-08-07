# Marvin Blockchain - Rust Implementation
===
[![Build Status](https://github.com/joaoh82/marvin-blockchain-rust/workflows/Rust/badge.svg)](https://github.com/joaoh82/marvin-blockchain-rust/actions)
[![dependency status](https://deps.rs/repo/github/joaoh82/marvin-blockchain-rust/status.svg)](https://deps.rs/repo/github/joaoh82/marvin-blockchain-rust)
[![Coverage Status](https://coveralls.io/repos/github/joaoh82/marvin-blockchain-rust/badge.svg?branch=main)](https://coveralls.io/github/joaoh82/marvin-blockchain-rust?branch=main)
[![Maintenance](https://img.shields.io/badge/maintenance-actively%20maintained-brightgreen.svg)](https://deps.rs/repo/github/joaoh82/marvin-blockchain-rust)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Welcome to the Rust implementation of the Marvin Blockchain. This project is part of a comparative study on building the same blockchain in both Go and Rust. Follow along as we explore the development process, performance, and features of a blockchain built in Rust.

## Project Overview

Marvin Blockchain is a distributed ledger and EVM Compatible inspired by Bitcoin and Ethereum, implemented in Rust. This project aims to provide a robust and scalable blockchain solution while comparing the nuances of building the same system in Go.

## Features

- **Proof of Work (PoW) Consensus Mechanism**
- **Peer-to-Peer (P2P) Networking**
- **Transaction and Block Validation**
- **Smart Contract Support via EVM Compatibility**
- **Comprehensive Unit Tests and Benchmarks**

## Getting Started

### Prerequisites

- Rust 1.80 or higher
- Git

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/joaoh82/marvin-blockchain-rust.git
    cd marvin-blockchain-rust
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

### Running the Blockchain
To start a node on the Marvin Blockchain:
```sh
./target/release/marvin-blockchain-rust
```

### Running Tests
To run the unit tests:
```sh
cargo test
```

### Project Progress (WIP)
- [ ] Add CLI support for ease of interaction
- [ ] Implement the basic blockchain data structure
- [ ] Proof of Work (PoW) consensus mechanism
- [ ] Peer-to-Peer (P2P) networking setup
- [ ] Basic transaction and block validation
- [ ] EVM integration for smart contract support
- [ ] JSON-RPC API implementation
- [ ] Advanced transaction handling and validation
- [ ] Enhanced security measures and best practices
- [ ] Performance benchmarking and optimization
- [ ] Comprehensive documentation and examples

## Roadmap (Subject to Change)
- [ ] Features and improvements planned for the project:
- [ ] Implement wallet functionalities
- [ ] Improve EVM compatibility and support
- [ ] Add more consensus mechanisms (e.g., PoS)
- [ ] Implement light client support
- [ ] Improve network protocol for better scalability
- [ ] Develop a robust test suite for security and performance
- [ ] Integration with Ethereum development tools
- [ ] Develop a block explorer
- [ ] Implement governance mechanisms
- [ ] Cross-chain interoperability solutions
- [ ] Improve documentation and developer guides

### Project Structure (WIP)
The project is structured as follows:
- `src/`: Contains the source code for the blockchain implementation.
- `src/blockchain/`: Contains the blockchain and block implementation.
- `src/network/`: Contains the networking implementation.
- `src/transaction/`: Contains the transaction implementation.
- `src/wallet/`: Contains the wallet implementation.
- `src/main.rs`: Contains the entry point for the blockchain application.
- `tests/`: Contains the unit tests for the blockchain implementation.

### Contributing
**Pull requests are warmly welcome!!!**

For major changes, please [open an issue](https://github.com/joaoh82/marvin-blockchain-rust/issues/new) first and let's talk about it. We are all ears!

If you'd like to contribute, please fork the repository and make changes as you'd like and shoot a Pull Request our way!

**Please make sure to update tests as appropriate.**

If you feel like you need it go check the GitHub documentation on [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

### Code of Conduct

Contribution to the project is organized under the terms of the
Contributor Covenant, the maintainer of SQLRite, [@joaoh82](https://github.com/joaoh82), promises to
intervene to uphold that code of conduct.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

### Contact
For any inquiries or support, please open an issue on Github or contact me at Joao Henrique Machado Silva <joaoh82@gmail.com>.
