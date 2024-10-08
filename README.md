# xChange
xChange is a Rust-based workspace for building a centralized cryptocurrency exchange (CEX).
Overview
xChange aims to provide a robust, scalable, and secure foundation for creating a centralized cryptocurrency exchange. This project leverages Rust's performance and safety features to ensure a reliable trading platform.
Features

- User account management
- Deposit and withdrawal functionality
- Order book management
- Trade execution engine
- Real-time market data streaming
- API for external integrations

## Project Structure
The workspace is organized into multiple crates, each responsible for a specific aspect of the exchange:

- crates/core: Core functionality for handling kycs and docs
- crates/engine: Order book management and matching engine
- crates/api: RESTful API for interacting with the exchange
- crates/notif_worker: Worker for handling notifications
- crates/subscriber: Real-time market data streaming from engine
- crates/market_maker: Market maker for providing liquidity
- crates/db: Database interactions and management
- crates/time_db: Time series database for storing historical market data
- crates/monitor: Monitoring and alerting system
- manifests: K8s Configuration files for the exchange

## Getting Started
### Prerequisites

Rust (latest stable version)
PostgreSQL (for data storage)
Redis (for caching and pub/sub)

### Installation

- Clone the repository:
```sh
  git clone https://github.com/8shaws/xChange.git
```
```sh
  cd xChange
```

- Build the project:
```sh
  cargo build --release
```

- Set up the database and configure environment variables (see Configuration section).
Run the exchange:
```sh
  cargo run --release
```
