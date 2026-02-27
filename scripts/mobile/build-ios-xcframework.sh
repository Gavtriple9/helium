#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT_DIR/mobile/ios/HeliumRust.xcframework"
HEADER_DIR="$ROOT_DIR/mobile/ios/include"

pushd "$ROOT_DIR" >/dev/null
cargo build --release --target aarch64-apple-ios -p helium
cargo build --release --target aarch64-apple-ios-sim -p helium
popd >/dev/null

rm -rf "$OUT_DIR"

xcodebuild -create-xcframework \
  -library "$ROOT_DIR/target/aarch64-apple-ios/release/libhelium.a" -headers "$HEADER_DIR" \
  -library "$ROOT_DIR/target/aarch64-apple-ios-sim/release/libhelium.a" -headers "$HEADER_DIR" \
  -output "$OUT_DIR"

echo "Created $OUT_DIR"
