#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
IOS_DIR="$ROOT_DIR/mobile/ios"
PROJECT_PATH="$IOS_DIR/Helium.xcodeproj"
XCFRAMEWORK_PATH="$IOS_DIR/HeliumRust.xcframework"

if ! command -v xcodegen >/dev/null 2>&1; then
  echo "xcodegen is required. Install with: brew install xcodegen"
  exit 1
fi

if [[ ! -d "$XCFRAMEWORK_PATH" ]]; then
  echo "Missing $XCFRAMEWORK_PATH"
  echo "Building XCFramework before generating project..."
  "$ROOT_DIR/scripts/mobile/build-ios-xcframework.sh"
fi

rm -rf "$PROJECT_PATH"

xcodegen generate \
  --spec "$IOS_DIR/project.yml" \
  --project "$IOS_DIR"

echo "Generated $PROJECT_PATH"

