#!/usr/bin/env bash
SEARCH_DIR="${1:-}"

if [ -z "$SEARCH_DIR" ]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

BINARY_NAME="cvm_integration_tests"
BIN_PATH="target/release/$BINARY_NAME"
LOG_DIR="cvm_integration_tests/logs"
METRICS_FILE="cvm_integration_tests/benchmarks/metrics.jsonl"

# Build once in release mode (faster, more realistic benchmarks)
echo "🔨 Building release binary..."
cargo build --release --quiet

# Check that the binary exists
if [ ! -x "$BIN_PATH" ]; then
  echo "Error: Compiled binary not found at $BIN_PATH"
  exit 1
fi

# Prepare output dirs
rm -rf "$LOG_DIR"
mkdir -p "$LOG_DIR"
: > "$METRICS_FILE"  # truncate

# Counters
total=0
passed=0

# Run on each .cvm file
while read -r file; do
  ((total++))
  base_name=$(basename "$file" .cvm)
  log_file="$LOG_DIR/${base_name}.log"

  # Run binary with metrics flag
  tmp_log=$(mktemp)
  "$BIN_PATH" --metrics "$file" > "$tmp_log" 2>&1

# Check if log starts with "Error"
  if head -n 1 "$tmp_log" | grep -q '^Error'; then
      echo "❌ $file → error (see $log_file)"
      mv "$tmp_log" "$log_file"
  else
      echo "✅ $file → ok"
      ((passed++))
  
      # Append entire tmp_log to metrics file
      cat "$tmp_log" >> "$METRICS_FILE"
  
      rm -f "$tmp_log"
  fi
done < <(find "$SEARCH_DIR" -type f -name "*.cvm" | sort)

# Final summary
echo "======================================"
echo "Finished running $total file(s)."
echo "✅ Passed: $passed"
echo "❌ Failed: $((total - passed))"
echo "📊 Metrics written to $METRICS_FILE"
echo "📝 Logs written to $LOG_DIR/"
