# WhatsMyIp

A simple command-line tool written in Rust to retrieve and display your external and private IP addresses. This project is part of my learning journey in Rust and provides a practical utility for terminal users on macOS, Linux, or Windows.

## Features

- Fetch and display your **external IP address** (via an online service).
- Retrieve and show your **private/local IP address** (from your network interface).
- Simple and clean command-line interface.
- Cross-platform support for macOS, Linux, and Windows.

## Installation

### Prerequisites

- Rust and Cargo installed. If not, follow the official [Rust installation guide](https://www.rust-lang.org/tools/install).

### Build from Source

1. Clone the repository:

   ```bash
   git clone https://github.com/rpucheta/whats-my-ip.git
   cd whats-my-ip
    ```

2.	Build the project using Cargo:

   ```bash
   cargo build --release
   ```
   
3.	The binary will be available in the target/release directory:

   ```bash
   ./target/release/whats-my-ip
   ```
