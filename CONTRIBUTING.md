# Contributing to Linuxstrap

First off, thank you for considering contributing to Linuxstrap! It's people like you that make open-source tools great.

## Development Environment Setup

To get started, you'll need the following installed:
- [Node.js](https://nodejs.org/) (v20 or higher recommended)
- [pnpm](https://pnpm.io/installation)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri Linux Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/linux/) (e.g., `libwebkit2gtk-4.1-dev`)

### Quick Start
1. Fork and clone the repository.
2. Run `pnpm install` to grab the frontend dependencies.
3. Run `pnpm run tauri dev` to start the app in development mode with hot-reloading.

## Code Structure

- `src/`: The frontend. We purposefully use **vanilla HTML, CSS, and JavaScript** to keep the app exceptionally fast and lightweight. Please avoid introducing heavy frameworks (like React, Vue, etc.) unless absolutely necessary and discussed beforehand.
- `src-tauri/src/lib.rs`: The backend Rust logic. Handles configuration reading/writing, applying mods to the Sober directory, and fetching patches.
- `src-tauri/assets/`: Included bundled modifications (Cursors, Sounds, etc.).

## Reporting Bugs & Suggesting Features

If you don't want to write code but have an idea or found a bug:
- Check the existing Issues to see if it has already been reported.
- Open a new Issue describing the bug or feature clearly.
- If it's a bug, please include your Linux distribution, your Sober version, and steps to reproduce the problem.

## Ai

Ai can be used just check the quality of you code before you create a pull request.