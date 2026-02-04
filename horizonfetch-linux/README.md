# HorizonFetch Linux

Modular system information fetch tool for Linux, written in Rust.

## Features

- **Modular architecture** - Clean separation of concerns (config, system, display)
- **Customizable** - Full control over colors, ASCII art, and displayed information
- **Fast** - Written in Rust for optimal performance
- **Cross-platform ready** - Uses standard Linux APIs

## Building

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build
```bash
cargo build --release
```

The binary will be at `target/release/horizonfetch`.

## Installation

```bash
# System-wide installation
sudo cp target/release/horizonfetch /usr/local/bin/

# User installation
mkdir -p ~/.local/bin
cp target/release/horizonfetch ~/.local/bin/
```

## Configuration

Create configuration file:
```bash
mkdir -p ~/.config/horizonfetch
cp hf.conf ~/.config/horizonfetch/
```

Edit `~/.config/horizonfetch/hf.conf` to customize:
- ASCII art
- Color scheme (ANSI codes)
- Which system information to display

## Project Structure

```
horizonfetch-linux/
├── src/
│   ├── main.rs      # Entry point
│   ├── config.rs    # Configuration handling
│   ├── system.rs    # System information gathering
│   └── display.rs   # Output rendering
├── Cargo.toml       # Dependencies
└── hf.conf          # Example configuration
```

## License

See the main repository LICENSE file.
