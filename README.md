# Marvin Blockchain - Rust Implementation

[![Build Status](https://github.com/joaoh82/marvin-blockchain-rust/workflows/Rust/badge.svg)](https://github.com/joaoh82/marvin-blockchain-rust/actions)
[![dependency status](https://deps.rs/repo/github/joaoh82/marvin-blockchain-rust/status.svg)](https://deps.rs/repo/github/joaoh82/marvin-blockchain-rust)
[![Coverage Status](https://coveralls.io/repos/github/joaoh82/marvin-blockchain-rust/badge.svg?branch=main)](https://coveralls.io/github/joaoh82/marvin-blockchain-rust?branch=main)
[![Maintenance](https://img.shields.io/badge/maintenance-actively%20maintained-brightgreen.svg)](https://deps.rs/repo/github/joaoh82/marvin-blockchain-rust)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Welcome to the Rust implementation of the Marvin Blockchain. This project is part of a comparative study on building the same blockchain in both Go and Rust. Follow along as we explore the development process, performance, and features of a blockchain built in Rust.

## Project Overview

Marvin Blockchain is a distributed ledger and EVM Compatible inspired by Bitcoin and Ethereum, implemented in Rust. This project aims to provide a robust and scalable blockchain solution while comparing the nuances of building the same system in Go.

### Read the series of posts about it:
##### Crafting a Blockchain in Go and Rust: A Comparative Journey
* [Part 0 - Introduction and Overview](https://medium.com/the-polyglot-programmer/crafting-a-blockchain-in-go-and-rust-a-comparative-journey-introduction-and-overview-part-0-e63dedee6b06)

## Features (WIP)

- **Proof of Work (PoW) Consensus Mechanism** (Subject to Change)
- **Peer-to-Peer (P2P) Networking**
- **Storage and Persistence**
- **Transaction and Block Validation**
- **Smart Contract Support via EVM Compatibility**
- **JSON-RPC API**
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
./target/release/marvin-blockchain
```

### Running Tests
To run the unit tests:
```sh
cargo test
```

### Project Progress (WIP)
- [x] Add CLI support for ease of interaction
- [x] Implemented key pair creation, sign and verify
- [x] Create key pair with mnemonic seed
- [x] Add address command to CLI
- [ ] Basic transaction and block validation
- [ ] Implement the basic blockchain data structure

### Roadmap (Subject to Change)
- [ ] Proof of Work (PoW) consensus mechanism
- [ ] Peer-to-Peer (P2P) networking implementation (transport layer)
- [ ] Storage and persistence for blockchain data
- [ ] EVM integration for smart contract support
- [ ] JSON-RPC API implementation
- [ ] Advanced transaction handling and validation
- [ ] Enhanced security measures and best practices
- [ ] Performance benchmarking and optimization
- [ ] Comprehensive documentation and examples
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
- `docs/`: Contains the documentation and guides for the project.
- `src/cli/`: Contains the CLI implementation for the blockchain.
- `src/core/`: Contains the core blockchain data structures and logic.
- `src/network/`: Contains the networking implementation.
- `src/transactions/`: Contains transaction handling and validation.
- `src/wallet/`: Contains the wallet implementation.
- `src/utils/`: Contains the utility functions and helpers.
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
Contributor Covenant, the maintainer of Marvin Blockchain, [@joaoh82](https://github.com/joaoh82), promises to
intervene to uphold that code of conduct.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

### Contact
For any inquiries or support, please open an issue on Github or contact me at Joao Henrique Machado Silva <joaoh82@gmail.com>.
