# linuxstrap

linuxstrap is a lightweight, native Linux configuration utility designed specifically for [Sober](https://vinegarhq.org) (Roblox on Linux). Built with Tauri (Rust and Vanilla JS/HTML/CSS), linuxstrap provides a seamless libadwaita-themed interface to manage your Sober settings, FastFlags, and game modifications.

Inspired by and integrating features from [Lution](https://github.com/Wookhq/Lution), [Silverr](https://github.com/Wookhq/silverr), [Fishtrap](https://github.com/fishstrap/fishstrap), and [Lucem](https://github.com/equinoxhq/lucem).

## Features

- **General Settings**: Easily configure Discord RPC, framerate limits, rendering backends (Vulkan/OpenGL), and performance tweaks like Feral GameMode.
- **FastFlags Management**: Built-in presets for lighting technology (Voxel, Shadowmap, Future), texture quality, MSAA, and player shadows. Includes a powerful custom FFlag editor.
- **Patch & Mod Engine**: Download and manage community patches or mods (2006/2013 Cursors, Classic Oof/Jump sounds, and 2014 Mobile Avatar Background).
- **Native Integration**: Directly modifies the local Sober Flatpak environment (`config.json` and `asset_overlay`) without needing manual file editing.

## Prerequisites

- Linux operating system.
- **Sober** installed via Flatpak (`org.vinegarhq.Sober`).

## Installation

1. Go to the [Releases](https://github.com/blazebsc/linuxstrap/releases) tab on GitHub.
2. Download the package matching your distribution (`.deb`, `.rpm`, or `.AppImage`).
3. Install or run the downloaded file.

## Building from Source

If you want to compile linuxstrap yourself, you will need Node.js and Rust installed on your system.

### 1. Install System Dependencies
On Debian/Ubuntu-based systems, install the required Tauri build dependencies:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```
*(For other distributions, check the [Tauri Prerequisites Guide](https://tauri.app/start/prerequisites/#linux))*

### 2. Clone and Build
```bash
git clone https://github.com/blazebsc/linuxstrap.git
cd linuxstrap

# Install frontend dependencies
pnpm install

# Run in development mode
pnpm run tauri dev

# Or build for release
pnpm run tauri build
```

## License

MIT License. See `LICENSE` for details.
