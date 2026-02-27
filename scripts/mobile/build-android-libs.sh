#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT_DIR/mobile/android/app/src/main/jniLibs"

if ! command -v cargo-ndk >/dev/null 2>&1; then
  echo "cargo-ndk is required. Install with: cargo install cargo-ndk"
  exit 1
fi

mkdir -p "$OUT_DIR"
rm -rf "$OUT_DIR/arm64-v8a" "$OUT_DIR/armeabi-v7a" "$OUT_DIR/x86_64"

pushd "$ROOT_DIR" >/dev/null
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o "$OUT_DIR" build --release -p helium
popd >/dev/null

echo "Android Rust libraries copied to $OUT_DIR"

