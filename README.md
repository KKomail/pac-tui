# pac-tui

---

## Overview

`pac-tui` is a modern, interactive terminal user interface (TUI) for Arch Linux's Pacman package manager. Built in Rust, it provides a more visual and intuitive way to manage your packages without leaving the terminal.

## Features (most of these are in progress)

- **Interactive Package Browsing**: Browse installed and available packages with a user-friendly interface
- **Search & Filter**: Quickly find packages by name or description
- **Package Management**: Install, remove, and upgrade packages with simple keybindings
- **System Updates**: View and perform system updates
- **Package Information**: Display detailed information about packages
- **Color-coded Output**: Easily distinguish between package states (installed, outdated, available)
- **Vim-like Navigation**: Familiar keybindings for efficient navigation
- **Responsive Design**: Adapts to terminal size changes

## Keybindings (Planned)

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Enter` | Select package / view details |
| `i` | Install selected package |
| `r` | Remove selected package |
| `u` | Upgrade system |
| `/` | Search packages |
| `q` | Quit |
| `?` | Show help |

## 🔧 Installation

### From Source

```bash
git clone https://github.com/KKomail/pac-tui.git
cd pac-tui
cargo build --release
sudo cp target/release/pac-tui /usr/local/bin/
```

Note: Will add to AUR (yay) in the future

### Prerequisites  

- Rust 1.70.0 or later  
- Cargo  
- Arch Linux (or any distro with Pacman)  
- Terminal with true color support (recommended)  

### Development  

This project is currently in active development.
*The core TUI framework is being built using:*  

- Ratatui  
- Crossterm  
- Tokio  

### Contributing  

Contributions are welcome!  
Feel free to open issues for bugs, feature requests, or questions!
