#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOURCE_IMAGE="${1:-$ROOT_DIR/assets/images/icon.png}"
ICONSET_DIR="$ROOT_DIR/target/macos-icon.iconset"
OUT_ICNS="$ROOT_DIR/mobile/macos/Helium/Helium.icns"
TMP_SQUARE="$ROOT_DIR/target/macos-icon-square.png"

if [[ ! -f "$SOURCE_IMAGE" ]]; then
  echo "Icon source not found: $SOURCE_IMAGE"
  exit 1
fi

if ! command -v sips >/dev/null 2>&1; then
  echo "sips is required but was not found"
  exit 1
fi

if ! command -v iconutil >/dev/null 2>&1; then
  echo "iconutil is required but was not found"
  exit 1
fi

mkdir -p "$ROOT_DIR/target"
rm -rf "$ICONSET_DIR"
mkdir -p "$ICONSET_DIR"

WIDTH=$(sips -g pixelWidth "$SOURCE_IMAGE" | awk '/pixelWidth/ {print $2}')
HEIGHT=$(sips -g pixelHeight "$SOURCE_IMAGE" | awk '/pixelHeight/ {print $2}')

if [[ -z "$WIDTH" || -z "$HEIGHT" ]]; then
  echo "Unable to determine icon dimensions for $SOURCE_IMAGE"
  exit 1
fi

if (( WIDTH < HEIGHT )); then
  EDGE=$WIDTH
else
  EDGE=$HEIGHT
fi

sips -c "$EDGE" "$EDGE" "$SOURCE_IMAGE" --out "$TMP_SQUARE" >/dev/null

sips -z 16 16 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_16x16.png" >/dev/null
sips -z 32 32 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_16x16@2x.png" >/dev/null
sips -z 32 32 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_32x32.png" >/dev/null
sips -z 64 64 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_32x32@2x.png" >/dev/null
sips -z 128 128 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_128x128.png" >/dev/null
sips -z 256 256 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_128x128@2x.png" >/dev/null
sips -z 256 256 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_256x256.png" >/dev/null
sips -z 512 512 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_256x256@2x.png" >/dev/null
sips -z 512 512 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_512x512.png" >/dev/null
sips -z 1024 1024 "$TMP_SQUARE" --out "$ICONSET_DIR/icon_512x512@2x.png" >/dev/null

iconutil -c icns "$ICONSET_DIR" -o "$OUT_ICNS"

echo "Created macOS icon: $OUT_ICNS"

