#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MACOS_DIR="$ROOT_DIR/mobile/macos"
PROJECT_PATH="$MACOS_DIR/Helium.xcodeproj"
DERIVED_DATA_DIR="$ROOT_DIR/target/macos-app-derived-data"
DIST_DIR="$ROOT_DIR/dist/macos"
APP_NAME="Helium.app"
APP_SOURCE="$DERIVED_DATA_DIR/Build/Products/Release/$APP_NAME"
APP_TARGET="$DIST_DIR/$APP_NAME"
ZIP_TARGET="$DIST_DIR/Helium-macos.app.zip"

"$ROOT_DIR/scripts/mobile/build-macos-xcframework.sh"
"$ROOT_DIR/scripts/mobile/generate-macos-icon.sh"
"$ROOT_DIR/scripts/mobile/generate-macos-project.sh"

rm -rf "$DERIVED_DATA_DIR"

xcodebuild \
  -project "$PROJECT_PATH" \
  -scheme Helium \
  -configuration Release \
  -derivedDataPath "$DERIVED_DATA_DIR" \
  CODE_SIGNING_ALLOWED=NO \
  build

if [[ ! -d "$APP_SOURCE" ]]; then
  echo "Expected app bundle not found at $APP_SOURCE"
  exit 1
fi

mkdir -p "$DIST_DIR"
rm -rf "$APP_TARGET" "$ZIP_TARGET"
cp -R "$APP_SOURCE" "$APP_TARGET"

ditto -c -k --sequesterRsrc --keepParent "$APP_TARGET" "$ZIP_TARGET"

echo "Created app bundle: $APP_TARGET"
echo "Created zip package: $ZIP_TARGET"
