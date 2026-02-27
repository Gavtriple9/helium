#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
IOS_DIR="$ROOT_DIR/mobile/ios"
PROJECT_PATH="$IOS_DIR/Helium.xcodeproj"

if ! command -v xcodegen >/dev/null 2>&1; then
  echo "xcodegen is required. Install with: brew install xcodegen"
  exit 1
fi

rm -rf "$PROJECT_PATH"

xcodegen generate \
  --spec "$IOS_DIR/project.yml" \
  --project "$IOS_DIR"

echo "Generated $PROJECT_PATH"

