#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MACOS_DIR="$ROOT_DIR/mobile/macos"
PROJECT_PATH="$MACOS_DIR/Helium.xcodeproj"
XCFRAMEWORK_PATH="$MACOS_DIR/HeliumRust.xcframework"

if ! command -v xcodegen >/dev/null 2>&1; then
  echo "xcodegen is required. Install with: brew install xcodegen"
  exit 1
fi

if [[ ! -d "$XCFRAMEWORK_PATH" ]]; then
  echo "Missing $XCFRAMEWORK_PATH"
  echo "Building XCFramework before generating project..."
  "$ROOT_DIR/scripts/mobile/build-macos-xcframework.sh"
fi

"$ROOT_DIR/scripts/mobile/generate-macos-icon.sh"

rm -rf "$PROJECT_PATH"

xcodegen generate \
  --spec "$MACOS_DIR/project.yml" \
  --project "$MACOS_DIR"

echo "Generated $PROJECT_PATH"

