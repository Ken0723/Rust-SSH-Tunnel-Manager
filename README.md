# 🚇 RTun - Rust SSH Tunnel Manager

> A lightweight CLI tool to manage SSH tunnels, written in Rust. Inspired by [mole](https://github.com/davrodpin/mole). This is a self-learning project, just feel free to use it. Also, welcome to you to comment of my code.

**RTun** (Rust SSH Tunnel Manager) is a command-line interface tool designed to simplify the creation and management of SSH local port forwarding tunnels.

This project was created as a hands-on exercise to **Rust**, focusing on:
*   CLI application architecture
*   Asynchronous I/O (`async`/`await`)
*   Configuration management
*   Network programming

## 🚧 Project Status

**Current State: Work In Progress (WIP)**

The SSH connection function is finished, and the next step is allow starting a tunnel in background and then check the running SSH tunnel with ```stats``` command

## ✨ Features

- [x] **Modern CLI**: User-friendly command-line interface built with `clap`.
- [x] **Modular Architecture**: Codebase organized into distinct modules (`cli`, `config`, `main`).
- [x] **Config Management**: Persistently save, load, list, delete tunnel configurations (TOML format).
- [X] **SSH Connection**: Establish secure SSH connections using `russh` & `tokio`.
- [X] **Port Forwarding**: Support for local port forwarding.
- [ ] **Run in background**: Make the ssh tunnel run in background, that allow create multi tunnel in single CLI
- [ ] **Grouping**: Support create a group for tunnel, that can allow up with group

## 🛠️ Installation & Build

### Prerequisites

To build this project, you need the Rust toolchain installed.

**For Windows Users:**
Since this project relies on low-level cryptographic libraries (`aws-lc-sys` via `russh`), you **must** have the following installed:

1.  **Visual Studio Build Tools** (Select "Desktop development with C++").
2.  **NASM & CMake** (Required for compiling the crypto bindings).

You can install these dependencies easily via PowerShell (`winget`):
```powershell
winget install Kitware.CMake
winget install NASM.NASM
# Note: After installation, ensure NASM is added to your system PATH.
```
### Building from Source
```
1. Clone the repository
git clone https://github.com/Ken0723/Rust-SSH-Tunnel-Manager.git
cd Rust-SSH-Tunnel-Manager

2. Update cargo
cargo update

3. Build the project
cargo build --release

4. Add to path
cargo install --path .
```

## 🚀 Usage
### During development, you can run the tool using cargo run.
### 1. Add a Tunnel
Save a new server configuration to your local config file.
#### Syntax: rtun add
```rtun add```
#### You can type rtm add with parameters
```rtun add {name} {local_port} {remote_host} {remote_port} {ssh_host} 22 ec2-user "{ssh-key-path}" false```
#### Or you can edit config.toml directly
```Format
[tunnels.Testing]
name = "Testing"
local_port = 1234
remote_host = "127.0.0.1"
remote_port = 5536
ssh_host = "123.123.123.123"
ssh_port = 22
ssh_user = "ec2-user"
ssh_key_path = ""
retry_on_failure = false
```

### 2. Start a Tunnel
Start the SSH tunnel using the saved configuration name.
#### Syntax: rtm up <NAME>
```rtun up Testing```

### 3. List all Tunnel
Start the SSH tunnel using the saved configuration name.
#### Syntax: rtm ls
```rtun ls```

## 📂 Project Structure
```
src/
├── cli.rs       # Interface: Defines command-line arguments and enums.
└── config.rs    # Data Layer: Handles Config struct definitions and File I/O.
├── handlers.rs  # Define the command
├── main.rs      # Entry point: Orchestrates CLI and Config modules.
└── ssh.rs       # Handle all SSH connection.
```

## 📚 Tech Stack
- Language: Rust 🦀
- CLI Parsing: clap
- Async Runtime: tokio
- SSH Protocol: russh (Planned)
- Serialization: serde & toml

## 📝 License
This project is licensed under the MIT License.