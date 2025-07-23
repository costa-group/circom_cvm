#!/bin/bash

SEARCH_DIR="$1"
BINARY_NAME="cvm_integration_tests"
BIN_PATH="target/debug/$BINARY_NAME"
LOG_DIR="logs"
FAILED_LOG="$LOG_DIR/failed.log"

if [ -z "$SEARCH_DIR" ]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

# Build the binary silently
cargo build --quiet

# Check that the binary exists
if [ ! -x "$BIN_PATH" ]; then
  echo "Error: Compiled binary not found at $BIN_PATH"
  exit 1
fi

# Create logs directory and clean previous failed log
mkdir -p "$LOG_DIR"
> "$FAILED_LOG"

# Run on each .cvm file
find "$SEARCH_DIR" -type f -name "*.cvm" | while read -r file; do
  base_name=$(basename "$file" .cvm)
  log_file="$LOG_DIR/${base_name}.log"

  "$BIN_PATH" "$file" > "$log_file" 2>&1

  # Check if log starts with "Error"
  if head -n 1 "$log_file" | grep -q '^Error'; then
    echo "❌ $file → error (see $log_file)"
    echo "$file" >> "$FAILED_LOG"
  else
    echo "✅ $file → ok (see $log_file)"
  fi
done
