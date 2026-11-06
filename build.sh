#!/usr/bin/env bash

# #
#   Ntfy Desktop build script (Tauri / Rust)
#
#   builds the installers for the operating system this script runs on.
#   Tauri cannot cross-compile desktop bundles; run it on every target OS
#   (or use the "Build Tauri" GitHub workflow).
#
#   needs:  Rust (https://rustup.rs)
#           Linux: libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
#                  librsvg2-dev libssl-dev libdbus-1-dev pkg-config
#
#   usage:  ./build.sh              run tests, then build installers
#           ./build.sh --debug      extra arguments are passed to `cargo tauri build`
#
#   the previous Electron build lives in build-electron.sh
# #

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${script_dir}/src-tauri"

echo
echo "============================================================================="
echo "                         NTFY DESKTOP BUILD SCRIPT"
echo "============================================================================="
echo

if ! command -v cargo >/dev/null 2>&1; then
    echo "[ERROR] Rust (cargo) not found. Install it from https://rustup.rs"
    exit 1
fi

if ! cargo tauri --version >/dev/null 2>&1; then
    echo "[INSTALL] Tauri CLI not found, installing (one time, takes a few minutes)..."
    cargo install tauri-cli --version "^2" --locked
fi

echo "[TEST] Running unit tests..."
cargo test --locked

echo "[BUILD] Building installers..."
cargo tauri build "$@"

echo
echo "[OK] Done. Installers are in: ${script_dir}/src-tauri/target/release/bundle"
