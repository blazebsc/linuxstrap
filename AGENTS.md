# linuxstrap

Tauri app (Rust + Vanilla JS/HTML/CSS) managing Sober Flatpak settings.

## Dev Commands

```bash
pnpm install
pnpm run tauri dev
pnpm run tauri build
# output: src-tauri/target/release/bundle/
```

## Flatpak

```bash
cd flatpak
./build.sh  # outputs linuxstrap_X.X.X_amd64.flatpak
```

Requires GNOME Platform 50 runtime.

## Key Paths

- **Sober config**: `~/.var/app/org.vinegarhq.Sober/config/sober/config.json`
- **Sober data**: `~/.var/app/org.vinegarhq.Sober/data/sober/`
- **Asset overlay**: `~/.var/app/org.vinegarhq.Sober/data/sober/asset_overlay/`
- **Linuxstrap config**: `~/.config/linuxstrap/config.json`
- **Rust source**: `src-tauri/src/` (modular)
- **Theme base assets**: `src-tauri/assets/theme_base/` (287+ Roblox UI icons for recoloring)

## Architecture

- **Rust backend** (modular):
  - `commands.rs` - Tauri commands
  - `sober_sync.rs` - Sober config sync
  - `mods_sync.rs` - Asset overlay sync
  - `mods_api.rs` - Fishstrap/GameBanana API
  - `zip_extractor.rs` - Zip extraction
  - `image_recolor.rs` - PNG recoloring for themes

- **Frontend**: Vanilla JS, no build step.
  - `window.__TAURI__.core.invoke()` - call Rust commands

## CI/Release

- GitHub Actions on main branch (`src-tauri/tauri.conf.json` change)
- Uses `pnpm` not npm
- Builds: `.deb`, `.rpm`, `.AppImage`, Flatpak
- Version from `src-tauri/tauri.conf.json` → `version`

## FFlags

**IMPORTANT**: As of September 2025, FFlags are locked down to official whitelist.

Texture Quality: 0-3 (not 0-4)
MSAA: 1, 2, 4

## Theme Generator

287+ Roblox UI icons recolored from `src-tauri/assets/theme_base/`.

## Mod System

- Mods: `~/.var/app/org.vinegarhq.Sober/data/sober/mods/`
- Types: `patch`, `fishstrap_themes`, `fastflag`

## Testing

No test suite. Manual verification via `pnpm run tauri dev`.