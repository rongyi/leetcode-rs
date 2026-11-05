#!/usr/bin/env bash
# Run one solution: ./scripts/run.sh 1
# Leading zeros are optional; problem 1 resolves to src/bin/0001.rs.
set -euo pipefail
cd "$(dirname "$0")/.."

if [ $# -ne 1 ]; then
  echo "usage: $(basename "$0") <problem-id>" >&2
  exit 64
fi

id=$(printf '%04d' "$((10#$1))")
if [ ! -f "src/bin/$id.rs" ]; then
  echo "no solution for problem $1 (expected src/bin/$id.rs)" >&2
  exit 1
fi

exec cargo run --quiet --bin "$id"
