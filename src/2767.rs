struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let sz = s.len();
        let mut dp = vec![sz as i32 + 1; sz + 1];
        dp[sz] = 0;
        let p5: BTreeSet<i32> = [1, 5, 25, 125, 625, 3125, 15625].into_iter().collect();
        for i in (0..sz).rev() {
            if s[i] == b'0' {
                continue;
            }
            let mut val = 0;
            for j in i..sz {
                val = val * 2 + (s[j] - b'0') as i32;
                if p5.contains(&val) {
                    dp[i] = dp[i].min(1 + dp[j + 1]);
                }
            }
        }
        if dp[0] > sz as i32 {
            return -1;
        }
        dp[0]
    }
}

fn main() {}
