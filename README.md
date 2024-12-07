# WhatsMyIp
A simple command-line tool written in Rust to retrieve and display your external and private IP addresses. This project is part of my learning journey in Rust and provides a practical utility for terminal users on macOS, Linux, or Windows.
This is just an extended project I created as part of my learning process with the Rust programming language. Hope it helps someone else too.

## Version
v1.0.0

## Features

- Fetch and display your **external IP address** (via an online service).
- Retrieve and show your **private/local IP address** (from your network interface).
- Copy your IP address directly to the clipboard.
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

## Usage
### Display your **public IP**:

Run the binary with the following commands:
   ```bash
   whatsmyip -public
   ```
Output example:
```
[WhatsMyIp] 🌍 Public IP: (copied to clipboard) 🗒️ ✅ -> 89.7.154.76
```

### Display your **private IP**:

Run the binary with the following commands:
   ```bash
   whatsmyip -private
   ```
Output example:
```
[WhatsMyIp] 🏠 Private IP: (copied to clipboard) 🗒️ ✅ -> 192.168.0.36
```

### Display both public and private IPs:

Run the binary with the following commands:
   ```bash
  whatsmyip
   ```
Output example:
```
[WhatsMyIp] 🌍 Public IP: 89.7.154.76
[WhatsMyIp] 🏠 Private IP: 192.168.0.36
[WhatsMyIp] Note: To copy an IP to the clipboard, use `-public` or `-private`
```

## Installing the whatsmyip Command
### macOS and Linux
1.	Build the Binary
Run the following command to build the binary:
```bash
cargo build --release
```
2. Move to Binary to a Directory in $PATH
```bash
sudo mv target/release/whatsmyip /usr/local/bin/whatsmyip 
```
3.	Verify the Installation
Run the command to verify the installation:
```bash
whatsmyip -public
```

### Windows
1.	Build the Binary
      Run the following command to build the binary:
```powershell
cargo build --release
```
2. Move the Binary to a Directory in %PATH%
Copy the binary to a directory that is part of your system’s %PATH%. For example, you can move it to the C:\Windows\System32 directory:
```powershell
copy target\release\whatsmyip.exe C:\Windows\System32\
```  
3. Verify the Installation
Open a new Command Prompt or PowerShell window and check if the binary is accessible:
```powershell
whatsmyip -public
```

## Notes
- When copying to the clipboard, the tool provides a confirmation in the output.
- The private IP is determined based on the network interface of the current machine.
- Clipboard functionality requires the clipboard crate and is supported across macOS, Linux, and Windows.

## Contributing
Contributions are welcome! Just remember thats a sample code to do some Rust learning but if you want, here’s how you can help:
1.	Fork this repository.
2.	Create a feature branch:
```bash
git checkout -b feature-name
```
3. Commit your changes:
```bash
git commit -m "Add new feature or fix bug"
```
4. Push to the branch:
```bash
git push origin feature-name
```
5. Submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](https://opensource.org/licenses/MIT) file for details.

## Acknowledgements
Special thanks to the Rust community and the authors of the crates used in this project: reqwest, local_ip_address, and clipboard.