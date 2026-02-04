# horizonfetch
[![Language](https://img.shields.io/badge/Rust-100%25-orange?style=plastic&logo=rust&logoColor=white&labelColor=5c5c5c)](https://github.com/horizonl1nux/horizonfetch/search?l=rust)
![release](https://img.shields.io/github/v/release/horizonl1nux/horizonfetch?label=release)
[![License: Apache 2.0](https://img.shields.io/badge/LICENSE-Apache_2.0-blue?style=plastic&logo=apache&logoColor=white&labelColor=5c5c5c)](https://opensource.org/licenses/Apache-2.0)
![Repo Size](https://img.shields.io/github/repo-size/horizonl1nux/horizonfetch)
[![photo](https://repology.org/badge/tiny-repos/horizonfetch.svg)](https://repology.org/project/horizonfetch/versions)

#### Forged in Rust, shaped by your style.

<div align="left">
  <a href="https://github.com/user-attachments/assets/82bb9031-42c7-4c55-9058-341b739b22fc">
    <img src="https://github.com/user-attachments/assets/82bb9031-42c7-4c55-9058-341b739b22fc" width="65%" style="margin: 8px; border-radius: 8px;" />
  </a>
</div>

## Installation

### Linux

#### Build from source
```bash
git clone https://github.com/horizonl1nux/horizonfetch.git
cd horizonfetch/horizonfetch-linux
cargo build --release
```

The compiled binary will be located at `target/release/horizonfetch`.

#### Install
```bash
# Copy to local bin
sudo cp target/release/horizonfetch /usr/local/bin/

# Or install for current user only
mkdir -p ~/.local/bin
cp target/release/horizonfetch ~/.local/bin/
```

### Windows
- [`winget`](https://github.com/microsoft/winget-pkgs/tree/ed987f873472e10012a9aafcdd4ee2bfea848ef7/manifests/h/Horizon/Horizonfetch/0.35-2):
  ```
  winget install horizonfetch
  ```
- Alternatively, download from the [Releases](https://github.com/horizonl1nux/horizonfetch/releases) tab

## Configuration

HorizonFetch can be customized via a configuration file.

### Linux
Create `~/.config/horizonfetch/hf.conf`:
```bash
mkdir -p ~/.config/horizonfetch
cp horizonfetch-linux/hf.conf ~/.config/horizonfetch/
```

Edit the file to customize colors, ASCII art, and which information to display.

### Windows
The configuration file is located at `%USERPROFILE%\horizonfetch\hf.config`.

See the example config files in `horizonfetch-linux/hf.conf` (Linux) or `horizonfetch-win/hf.conf` (Windows) for all available options.
## Running the Application

You can launch `horizonfetch` in several convenient ways:

- **From the folder directly:**  
  Navigate to the folder containing the binary and run:  
  ```bash
  ./horizonfetch
  ```

- **Via File Explorer:**  
  Simply double-click the executable file.

- **From anywhere in terminal:**  
  If installed via `winget` or added to your system’s `$PATH`, you can run it globally:  
  ```bash
  horizonfetch
  ```
## Flags and Options

To use flags, run the binary with the desired option:

```bash
./horizonfetch -help
```

If `horizonfetch` is in your `$PATH`, you can use it directly:


```bash
horizonfetch -help
```
😎You can use both short (-flag) and long (--flag) versions for most options:
```
-help  or --help
```
😈For more advanced flags and new customization options, stay tuned — version `0.36-1` is coming soon...

## Customization
Soon...

  <a href="https://repology.org/project/horizonfetch/versions">
    <img src="https://repology.org/badge/vertical-allrepos/horizonfetch.svg?columns=2" alt="Repology status" height="45">
  </a>
</h3>
