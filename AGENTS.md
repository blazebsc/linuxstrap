# linuxstrap

Tauri app (Rust + Vanilla JS/HTML/CSS) managing Sober Flatpak settings.

## Dev Commands

```bash
pnpm install
pnpm run tauri dev
pnpm run tauri build
# output: src-tauri/target/release/bundle/
```

## Key Paths

- **Sober config**: `~/.var/app/org.vinegarhq.Sober/config/sober/config.json`
- **Sober data**: `~/.var/app/org.vinegarhq.Sober/data/sober/`
- **Asset overlay**: `~/.var/app/org.vinegarhq.Sober/data/sober/asset_overlay/`
- **Linuxstrap config**: `~/.config/linuxstrap/config.json`
- **Rust source**: `src-tauri/src/` (modular)
- **Theme base assets**: `src-tauri/assets/theme_base/` (287+ Roblox UI icons for recoloring)

## Architecture

- **Rust backend** (modular):
  - `config.rs` - `LinuxstrapConfig` struct
  - `commands.rs` - Tauri commands (`get_config`, `save_config`, `generate_theme`, etc.)
  - `sober_sync.rs` - Sober config sync with FFlag presets
  - `mods_sync.rs` - Asset overlay sync (fonts, cursors, sounds, themes)
  - `mods_api.rs` - Fishstrap/GameBanana API
  - `zip_extractor.rs` - Zip extraction
  - `image_recolor.rs` - PNG recoloring for theme generation

- **Frontend**: Vanilla JS, no build step.
  - `window.__TAURI__.core.invoke()` - call Rust commands
  - `window.__TAURI__.event` - listen for progress events

## Asset Overlay Convention

The `asset_overlay` directory mirrors `packages/com.roblox.client/base.apk/assets`.
Files must recreate the folder structure exactly. Example:
```
~/.var/app/org.vinegarhq.Sober/data/sober/asset_overlay/
├── content/textures/Cursors/KeyboardMouse/...
└── ExtraContent/LuaPackages/Packages/_Index/...
```

## FFlags

**IMPORTANT**: As of September 2025, FFlags are locked down to an official whitelist.
Flags not on the allowlist will be ignored.

Texture Quality values: 0-3 (not 0-4)
MSAA values: 1, 2, 4 (sample count)
Geometry LOD flags: DFIntCSGLevelOfDetailSwitchingDistance variants

## Theme Generator

The app includes a theme generator that recolors 287+ Roblox UI icons:
1. Base icons stored in `src-tauri/assets/theme_base/`
2. User selects a color preset or custom hex color
3. "Apply Theme" button triggers `generate_theme` command
4. Progress bar shows processing status via Tauri events
5. Recolored icons copied to `asset_overlay/content/` and `asset_overlay/ExtraContent/`

## Mod System

- Mods: `~/.var/app/org.vinegarhq.Sober/data/sober/mods/`
- Manifest: `{ "id": "modname", "type": "patch", "files": [...] }`
- Types: `patch`, `fishstrap_themes`, `fastflag`
- Fishstrap themes extract to `assets/{theme_name}/`

## Tuxstrap Features (FFlag presets)

- **Super Performance Mode**: Voxel rendering, grass disabled, shadows off, texture quality 0, SSAO off
- **Network Optimization**: OptimizeNetwork, OptimizeNetworkTransport, MTU 900, NewInput
- **Wayland Clipboard**: FFlagClientAllowClipboardControl, FFlagClientAllowDBus, FFlagIsLinux
- **Bring Back OOF**: FFlagDisableFeedbackSoothsayerCheck (default: true)

## Release Build

CI on main branch → `npm run tauri build`. Publishes `.deb`, `.AppImage`, `.rpm`.
Version from `src-tauri/tauri.conf.json` → `version`.

## Troubleshooting Features

The app includes a Troubleshooting section with:
- **Kill Sober**: `flatpak kill org.vinegarhq.Sober` (zombie processes)
- **Wake GPU**: Runs `vkcube`/`vulkaninfo` to wake discrete NVIDIA GPU before launch
- **Fix XDG Portals**: Configures `xdg-desktop-portal` for Roblox link joining (Hyprland, etc.)
- **SSE4.2 Check**: Detects CPU support (Intel 2008+, AMD 2013+)
- **Sober Status**: Live monitoring of running Sober processes
- **Audio Driver Override**: Switch between PulseAudio, PipeWire, ALSA via Flatpak env
- **Known Issues**: 9 collapsible entries covering OOM/RBXCRASH, SSE4.2 launch failure, vkGetPhysicalDeviceSurfacePresentModesKHR, FFlag whitelist, HiDPI on X11, audio crackling, browser/Roblox links, pixelated outlines, GPU detection

## Testing

No test suite. Manual:
- `pnpm run tauri dev`
- Verify: settings save/load, theme generation, mods install/uninstall, launch Sober

---

## Progress

### Done
- Tauri 2 rewrite with modular Rust backend
- GameBanana API v11 integration (Roblox Game ID: 2879)
- Fishstrap mods support
- Zip extraction with nested stripping
- File/folder picker commands
- Custom font installation via `ttf-parser`
- Theme generator (287 icons recolored)
- Theme presets + custom hex picker
- Config persistence with Sober import on first run
- FFlag validation via MaximumADHD tracker
- Tuxstrap FFlag presets
- Troubleshooting section (Kill Sober, Wake GPU, Fix XDG Portals, SSE4.2, Sober status, audio driver)
- 9 known issues with solutions

### Next
- Add bootstrapper icon selection