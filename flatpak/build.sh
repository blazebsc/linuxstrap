#!/bin/bash
set -e

cd "$(dirname "$0")"

VERSION=$(node -p "require('../src-tauri/tauri.conf.json').version")

FLATPAK_FILE="$(pwd)/linuxstrap_${VERSION}_amd64.flatpak"

echo "Building Flatpak..."

# Clean previous build
rm -rf build-dir .flatpak-builder

# Build
flatpak run --user org.flatpak.Builder build-dir org.linuxstrap.dev.yml

# Export and create bundle
mkdir -p repo
flatpak --user build-export repo build-dir
flatpak --user build-bundle repo "$FLATPAK_FILE" org.linuxstrap.dev

echo "Done!"