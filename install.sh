#!/usr/bin/env sh

set -o errexit

version=v0.0.0
name=nvim-conventional-commits

cargo_build() {
    if command -v cargo > /dev/null; then
        echo "Trying to build locally using Cargo.."
        cargo build --release
    else
        echo "Could not build binary. Your installation might be corrupt."
        return 1
    fi
}

build_binary() {
  cargo_build || echo "Prebuilt binaries are not ready for this platform."
}

arch=$(uname)
case "${arch}" in
    "Darwin") build_binary ;;
    *) echo "No pre-built binary available for ${arch}."; cargo_build ;;
esac
