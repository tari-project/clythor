# Clythor - Merge Mining Tari and Monero with RandomX

[Clythor](https://github.com/tari-project/clythor) is a lightweight RandomX miner designed specifically for merge mining
Tari and Monero. Clythor is intentionally minimal and optimized for performance, offloading merge mining
responsibilities to an external proxy or base node. The miner itself handles mining RandomX hashes but is agnostic to
the complexities of merge mining.

## Features

- **RandomX mining** for both Monero and Tari.
- **Lightweight** with minimal dependencies.
- **Merge mining support** through an external proxy or base node.
- Configurable **number of threads** for optimized mining.
- **HTTP server** for monitoring and control.

## Requirements

Before running the miner, ensure you have:

- A **Monero wallet address** for payouts.
- A **Monero base node address** or Tari MergeMiningProxy for block templates.

## Getting Started

### Install

To install Clythor, clone the repository and compile the miner using `cargo`:

```bash
git clone https://github.com/tari-project/clythor
cd clythor
cargo build --release
```

### Usage

Run the miner with your desired settings. At minimum, you'll need to provide a **Monero base node address** and a *
*Monero wallet address**:

```bash
./target/release/clythor --user YOUR_MONERO_WALLET_ADDRESS --monero-base-node-address TARI_MERGE_MINING_PROXY 
```

## Merge Mining

Clythor is designed to integrate with a proxy or base node for handling merge mining with Tari. For more details on
setting up a merge mining proxy, visit the [Tari Merge Mining](https://github.com/tari-project/tari) documentation.
