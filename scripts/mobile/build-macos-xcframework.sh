#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT_DIR/mobile/macos/HeliumRust.xcframework"
HEADER_DIR="$ROOT_DIR/mobile/macos/include"
UNIVERSAL_DIR="$ROOT_DIR/target/macos-universal"
UNIVERSAL_LIB="$UNIVERSAL_DIR/libhelium.a"

pushd "$ROOT_DIR" >/dev/null
cargo build --release --target aarch64-apple-darwin -p helium
cargo build --release --target x86_64-apple-darwin -p helium
popd >/dev/null

rm -rf "$OUT_DIR"
rm -rf "$UNIVERSAL_DIR"
mkdir -p "$UNIVERSAL_DIR"

lipo -create \
  "$ROOT_DIR/target/aarch64-apple-darwin/release/libhelium.a" \
  "$ROOT_DIR/target/x86_64-apple-darwin/release/libhelium.a" \
  -output "$UNIVERSAL_LIB"

xcodebuild -create-xcframework \
  -library "$UNIVERSAL_LIB" -headers "$HEADER_DIR" \
  -output "$OUT_DIR"

echo "Created $OUT_DIR"

