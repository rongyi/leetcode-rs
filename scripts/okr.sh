#!/usr/bin/env bash
# "One key run": how many problems did I touch today / this week?
set -uo pipefail
cd "$(dirname "$0")/.."

count() {
  git log "$@" --name-only --pretty=format:'' \
    | grep -E '^src/bin/[0-9]+\.rs$' \
    | sort -u
}

today=$(count --since=midnight)
week=$(count --since='7 days ago')

printf '%s\n' "$today" | grep . || true
echo "today: $(printf '%s\n' "$today" | grep -c .) leetcode in rust, happy coding ry!"
echo "last 7 days: $(printf '%s\n' "$week" | grep -c .) leetcode in rust, happy coding ry!"
