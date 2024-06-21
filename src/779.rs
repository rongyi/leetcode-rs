#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        // odd index is same as parent
        if k & 1 == 1 {
            return Self::kth_grammar(n - 1, (k + 1) / 2);
        }
        let val = Self::kth_grammar(n - 1, (k + 1) / 2);
        if val == 0 {
            return 1;
        }
        return 0;
    }
}

fn main() {}
