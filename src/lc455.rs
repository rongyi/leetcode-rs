struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut i = 0;
        let mut j = 0;
        let mut cnt = 0;
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                cnt += 1;
                i += 1;
            }
            // 1. consumed, we shift to next
            // 2. not consumed, we shift to next to give g[i] another try
            j += 1;
        }

        cnt
    }
}

fn main() {}
