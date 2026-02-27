#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT_DIR/mobile/ios/HeliumRust.xcframework"
HEADER_DIR="$ROOT_DIR/mobile/ios/include"
UNIVERSAL_SIM_DIR="$ROOT_DIR/target/ios-sim-universal"
UNIVERSAL_SIM_LIB="$UNIVERSAL_SIM_DIR/libhelium.a"

pushd "$ROOT_DIR" >/dev/null
cargo build --release --target aarch64-apple-ios -p helium
cargo build --release --target aarch64-apple-ios-sim -p helium
cargo build --release --target x86_64-apple-ios -p helium
popd >/dev/null

rm -rf "$OUT_DIR"
rm -rf "$UNIVERSAL_SIM_DIR"
mkdir -p "$UNIVERSAL_SIM_DIR"

lipo -create \
  "$ROOT_DIR/target/aarch64-apple-ios-sim/release/libhelium.a" \
  "$ROOT_DIR/target/x86_64-apple-ios/release/libhelium.a" \
  -output "$UNIVERSAL_SIM_LIB"

xcodebuild -create-xcframework \
  -library "$ROOT_DIR/target/aarch64-apple-ios/release/libhelium.a" -headers "$HEADER_DIR" \
  -library "$UNIVERSAL_SIM_LIB" -headers "$HEADER_DIR" \
  -output "$OUT_DIR"

echo "Created $OUT_DIR"

