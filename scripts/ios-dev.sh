#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "error: iOS development requires macOS with the full Xcode app installed." >&2
  exit 1
fi

command -v bun >/dev/null 2>&1 || {
  echo "error: bun is required." >&2
  exit 1
}

rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

bun install --no-save
(cd client && bun install --no-save)

if [[ ! -d src-tauri/gen/apple || "${LQXP_FORCE_IOS_INIT:-}" == "1" ]]; then
  bun run ios:init
fi

bun tauri ios dev "$@"
