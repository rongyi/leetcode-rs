#!/usr/bin/env bash
# Type-check solutions.
#   ./scripts/check.sh          -> everything, with a summary of what is broken
#   ./scripts/check.sh 1 42 300 -> just those problems
set -euo pipefail
cd "$(dirname "$0")/.."

if [ $# -gt 0 ]; then
  bins=()
  for id in "$@"; do
    bins+=(--bin "$(printf '%04d' "$((10#$id))")")
  done
  exec cargo check "${bins[@]}"
fi

total=$(ls src/bin | wc -l | tr -d ' ')
mkdir -p target
log=target/check.log

set +e
cargo check --quiet --bins --keep-going --message-format=short >"$log" 2>&1
status=$?
set -e

if [ "$status" -eq 0 ]; then
  rm -f "$log"
  echo "all $total solutions type-check"
  exit 0
fi

broken=$(grep -E '^src/bin/[0-9]+\.rs:[0-9]+:[0-9]+: error' "$log" | cut -d: -f1 | sort -u)
echo "broken solutions ($(printf '%s\n' "$broken" | grep -c .) of $total):"
echo
printf '%s\n' "$broken" | while read -r file; do
  grep -m1 "^$file:" "$log" || true
done
echo
echo "full cargo log: $log"
exit 1
