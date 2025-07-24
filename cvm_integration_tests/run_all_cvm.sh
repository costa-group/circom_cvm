#!/bin/bash

SEARCH_DIR="$1"
BINARY_NAME="cvm_integration_tests"
BIN_PATH="target/debug/$BINARY_NAME"
LOG_DIR="logs"

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

# Counters
total=0
passed=0

# Run on each .cvm file
while read -r file; do
  ((total++))
  base_name=$(basename "$file" .cvm)
  log_file="$LOG_DIR/${base_name}.log"

  # Run and capture output to a temporary file
  tmp_log=$(mktemp)
  "$BIN_PATH" "$file" > "$tmp_log" 2>&1

  # Check if log starts with "Error"
  if head -n 1 "$tmp_log" | grep -q '^Error'; then
    echo "❌ $file → error (see $log_file)"
    mv "$tmp_log" "$log_file"
  else
    echo "✅ $file → ok"
    ((passed++))
    rm -f "$tmp_log"
  fi
done < <(find "$SEARCH_DIR" -type f -name "*.cvm" | sort)

# Final summary
echo "======================================"
echo "Finished running $total test(s)."
echo "✅ Passed: $passed"
echo "❌ Failed: $((total - passed))"
