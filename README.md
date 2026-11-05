# leetcode — Rust solutions

Rust solutions for LeetCode, one file per problem. Started in 2023 as a way to
learn Rust the hard way, and still going.

## Layout

```
src/
├── main.rs        # scratch pad — the default `cargo run` target, not a solution
├── lib.rs         # shared crate root (only exposes the prelude)
├── prelude.rs     # TreeNode / ListNode, the types LeetCode injects for you
└── bin/           # <- every solution lives here, one binary per problem
    ├── 0001.rs
    ├── 0002.rs
    └── ...

legacy/            # 463 older drafts (formerly lcNN.rs, 2023-12 .. 2024-04),
                   # kept for the record but not part of the build
scripts/           # run / check / new / okr / reverse helpers
```

Each file in `src/bin/` is its own Cargo binary target, so:

- **nothing has to be registered** in `main.rs` or `lib.rs` — drop a file in and
  it is immediately buildable. No `mod` list to maintain.
- every solution compiles **in isolation**, so one broken/experimental file never
  breaks the rest of the repo.
- no name clashes between the `struct Solution` that every problem declares.

File names are the problem id, zero-padded to 4 digits so the directory sorts
numerically. `1.rs` and `0001.rs` are the same problem.

## Daily use

```sh
./scripts/new.sh 2000        # scaffold src/bin/2000.rs from a template
./scripts/run.sh 1           # cargo run --bin 0001
./scripts/check.sh 1 42 300  # type-check just those problems
./scripts/check.sh           # type-check all 1779 and list whatever is broken
./scripts/okr.sh             # how many problems touched today / this week
./scripts/reverse.sh 1       # copy a solution into ../demo/src/main.rs
```

Raw cargo works too:

```sh
cargo run --bin 0001
cargo check --bin 0001
cargo run                    # the scratch pad in src/main.rs
```

Always name the target: a bare `cargo build` links all 1779 binaries at once,
which is slow and rarely what you want.

## Conventions

A solution file is self-contained and looks like this:

```rust
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // ...
    }
}

fn main() {}
```

`fn main` is optional but handy when the file is a binary target (a bin without
`main` does not build). Keep experiments in `main()` so a file can be run
directly.

### Shared types

LeetCode silently provides `TreeNode`, `ListNode`, `Rc` and `RefCell` for tree
and linked-list problems. Here they live in `src/prelude.rs`; add

```rust
use lc::prelude::*;
```

to the top of a file that needs them. (The 24 files that used to rely on the
implicit definitions now do this.)

### Interactive problems

Problems judged through a callback (`Master::guess`, `isBadVersion`, `rand7`,
`guess`) cannot compile on their own. Those files carry a local stub marked
`// LeetCode injects the judge API; stubbed so the file type-checks.` — wire up
a real judge if you want to run them.

### Notes on individual files

- Files with an `_alt` suffix (or an inner `mod ai { struct Solution; ... }`)
  hold a second attempt at the same problem, kept for comparison.

## Status

- 1779 solutions in `src/bin/`, all of them type-check (`./scripts/check.sh`).
- 463 legacy drafts in `legacy/`, written before the per-file convention and not
  compiled. They are superseded by the matching `src/bin/NNNN.rs` for 415 ids;
  the rest are the only version on file.
