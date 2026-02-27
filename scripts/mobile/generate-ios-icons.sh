#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOURCE_IMAGE="${1:-$ROOT_DIR/assets/images/icon.png}"
ASSETS_DIR="$ROOT_DIR/mobile/ios/Helium/Assets.xcassets"
APPICONSET_DIR="$ASSETS_DIR/AppIcon.appiconset"
TMP_SQUARE="$ROOT_DIR/target/ios-icon-square.png"

if [[ ! -f "$SOURCE_IMAGE" ]]; then
  echo "Icon source not found: $SOURCE_IMAGE"
  exit 1
fi

if ! command -v sips >/dev/null 2>&1; then
  echo "sips is required but was not found"
  exit 1
fi

mkdir -p "$ROOT_DIR/target"
rm -rf "$APPICONSET_DIR"
mkdir -p "$APPICONSET_DIR"
mkdir -p "$ASSETS_DIR"

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

write_icon() {
  local size="$1"
  local filename="$2"
  sips -z "$size" "$size" "$TMP_SQUARE" --out "$APPICONSET_DIR/$filename" >/dev/null
}

write_icon 40 icon-20@2x.png
write_icon 60 icon-20@3x.png
write_icon 58 icon-29@2x.png
write_icon 87 icon-29@3x.png
write_icon 80 icon-40@2x.png
write_icon 120 icon-40@3x.png
write_icon 120 icon-60@2x.png
write_icon 180 icon-60@3x.png

write_icon 20 icon-20@1x-ipad.png
write_icon 40 icon-20@2x-ipad.png
write_icon 29 icon-29@1x-ipad.png
write_icon 58 icon-29@2x-ipad.png
write_icon 40 icon-40@1x-ipad.png
write_icon 80 icon-40@2x-ipad.png
write_icon 76 icon-76@1x-ipad.png
write_icon 152 icon-76@2x-ipad.png
write_icon 167 icon-83.5@2x-ipad.png

write_icon 1024 icon-1024.png

cat > "$ASSETS_DIR/Contents.json" <<'JSON'
{
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}
JSON

cat > "$APPICONSET_DIR/Contents.json" <<'JSON'
{
  "images" : [
    { "idiom" : "iphone", "size" : "20x20", "scale" : "2x", "filename" : "icon-20@2x.png" },
    { "idiom" : "iphone", "size" : "20x20", "scale" : "3x", "filename" : "icon-20@3x.png" },
    { "idiom" : "iphone", "size" : "29x29", "scale" : "2x", "filename" : "icon-29@2x.png" },
    { "idiom" : "iphone", "size" : "29x29", "scale" : "3x", "filename" : "icon-29@3x.png" },
    { "idiom" : "iphone", "size" : "40x40", "scale" : "2x", "filename" : "icon-40@2x.png" },
    { "idiom" : "iphone", "size" : "40x40", "scale" : "3x", "filename" : "icon-40@3x.png" },
    { "idiom" : "iphone", "size" : "60x60", "scale" : "2x", "filename" : "icon-60@2x.png" },
    { "idiom" : "iphone", "size" : "60x60", "scale" : "3x", "filename" : "icon-60@3x.png" },

    { "idiom" : "ipad", "size" : "20x20", "scale" : "1x", "filename" : "icon-20@1x-ipad.png" },
    { "idiom" : "ipad", "size" : "20x20", "scale" : "2x", "filename" : "icon-20@2x-ipad.png" },
    { "idiom" : "ipad", "size" : "29x29", "scale" : "1x", "filename" : "icon-29@1x-ipad.png" },
    { "idiom" : "ipad", "size" : "29x29", "scale" : "2x", "filename" : "icon-29@2x-ipad.png" },
    { "idiom" : "ipad", "size" : "40x40", "scale" : "1x", "filename" : "icon-40@1x-ipad.png" },
    { "idiom" : "ipad", "size" : "40x40", "scale" : "2x", "filename" : "icon-40@2x-ipad.png" },
    { "idiom" : "ipad", "size" : "76x76", "scale" : "1x", "filename" : "icon-76@1x-ipad.png" },
    { "idiom" : "ipad", "size" : "76x76", "scale" : "2x", "filename" : "icon-76@2x-ipad.png" },
    { "idiom" : "ipad", "size" : "83.5x83.5", "scale" : "2x", "filename" : "icon-83.5@2x-ipad.png" },

    { "idiom" : "ios-marketing", "size" : "1024x1024", "scale" : "1x", "filename" : "icon-1024.png" }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}
JSON

echo "Created iOS app icon set at $APPICONSET_DIR"

