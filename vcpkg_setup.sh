#!/usr/bin/env bash
set -e

# Install cargo-vcpkg if missing
if ! command -v cargo-vcpkg >/dev/null 2>&1; then
  cargo install cargo-vcpkg
fi

cd engine
cargo vcpkg -v build
