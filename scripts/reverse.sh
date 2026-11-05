#!/usr/bin/env bash
# Copy a solution into the separate `../demo` scratch crate as its main.rs.
# Note: solutions that start with `use lc::prelude::*;` need that crate to exist,
# so prefer `./scripts/run.sh <id>` for everyday runs.
set -euo pipefail
cd "$(dirname "$0")/.."

if [ $# -ne 1 ]; then
  echo "usage: $(basename "$0") <problem-id>" >&2
  exit 64
fi

id=$(printf '%04d' "$((10#$1))")
src="src/bin/$id.rs"
dst="../demo/src/main.rs"

if [ ! -f "$src" ]; then
  echo "no solution for problem $1 (expected $src)" >&2
  exit 1
fi
if [ ! -d "$(dirname "$dst")" ]; then
  echo "scratch crate not found: $dst" >&2
  exit 1
fi

cp "$src" "$dst"
echo "$src -> $dst"
