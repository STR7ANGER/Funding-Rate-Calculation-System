#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "Cleaning build artifacts and temporary files in: ${ROOT_DIR}"

remove_path() {
  local p="$1"
  if [ -e "$p" ]; then
    echo "- Removing: $p"
    rm -rf "$p"
  else
    echo "- Skipping (not found): $p"
  fi
}

# Build outputs
remove_path "${ROOT_DIR}/target"
remove_path "${ROOT_DIR}/programs/funding-rate/target"

# Local validator/test ledger data (regenerable)
remove_path "${ROOT_DIR}/test-ledger"

# Common junk files
echo "- Deleting common junk files (*.log, .DS_Store, *.tmp, swap files)"
find "${ROOT_DIR}" \
  -name ".DS_Store" -or \
  -name "*.log" -or \
  -name "*.tmp" -or \
  -name "*.swp" -or \
  -name "*.swo" \
  -print -delete || true

echo "Cleanup completed."


