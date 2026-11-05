#!/usr/bin/env bash
# Start a new solution: ./scripts/new.sh 2000
set -euo pipefail
cd "$(dirname "$0")/.."

if [ $# -ne 1 ]; then
  echo "usage: $(basename "$0") <problem-id>" >&2
  exit 64
fi

id=$(printf '%04d' "$((10#$1))")
file="src/bin/$id.rs"

if [ -f "$file" ]; then
  echo "$file already exists, not overwriting" >&2
  exit 1
fi

cat >"$file" <<'RUST'
struct Solution;

impl Solution {
    pub fn solve(nums: Vec<i32>) -> i32 {
        nums.len() as i32
    }
}

fn main() {
    println!("{}", Solution::solve(vec![1, 2, 3]));
}
RUST

echo "created $file"
