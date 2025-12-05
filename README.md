# Paradox Checksum Patcher

A tool that patches Paradox game executables to bypass ironman checksum validation, allowing you to use mods that change checksum while still earning achievements.

**Note:** This patcher does NOT enable console commands or achievement-disabling game rules in ironman mode.

## Important

The patcher only modifies the current game executable. If Paradox releases a game update, you'll need to run the patcher again.

## Installation

### Option 1: Download Pre-built Binary

Download the latest `paradox-checksum-patcher.exe` from [Releases](https://github.com/DaniPopes/paradox-checksum-patcher/releases)

### Option 2: Build from Source

**Prerequisites:** Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install)

```bash
# Clone the repository
git clone https://github.com/DaniPopes/paradox-checksum-patcher.git
cd paradox-checksum-patcher

# Build release binary
cargo build --release

# Binary will be at: target/release/paradox-checksum-patcher[.exe]
```

## Usage

### Option 1: Place in Game Directory

1. Place `paradox-checksum-patcher.exe` in your game directory (Steam: Right-click game → Manage → Browse local files)
2. Run `paradox-checksum-patcher.exe`

The patcher will automatically detect and patch supported game executables in the directory.

### Option 2: Run from Command Line

```bash
# Auto-detect games in current directory
./paradox-checksum-patcher.exe

# Patch specific file
./paradox-checksum-patcher.exe path/to/eu4.exe

# Patch multiple files or directories
./paradox-checksum-patcher.exe ./eu4.exe ~/.local/share/Steam/steamapps/common/Europa\ Universalis\ V/binaries/
```

## Supported Games and Platforms

|                       | Windows | Linux | macOS |
|-----------------------|---------|-------|-------|
| Europa Universalis IV | ✓       | ✗     | ✗     |
| Europa Universalis V  | ✓       | ✗     | ✗     |
| Hearts of Iron IV     | ✓       | ✗     | ✗     |

**Note:** The table above refers to native game executables.
(Note that EU5 doesn't have native Linux/macOS binaries)
However, the patcher itself is cross-platform and can patch Windows executables from Linux/macOS, making it usable with Proton.
