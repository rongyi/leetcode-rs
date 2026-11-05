struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut memo = HashMap::new();

        // k = 26 is a special case: even with a change,
        // you can't exceed 26 distinct chars, so it's always 1 partition.
        if k == 26 {
            return 1;
        }

        fn solve(
            idx: usize,
            mask: i32,
            has_changed: bool,
            k: i32,
            s: &[u8],
            memo: &mut HashMap<(usize, i32, bool), i32>,
        ) -> i32 {
            if idx == s.len() {
                return 1;
            }
            let state = (idx, mask, has_changed);
            if let Some(&res) = memo.get(&state) {
                return res;
            }
            let mut ret = 0;
            // 1. dont change
            let change_bit = 1 << (s[idx] - b'a') as i32;
            let new_mask = mask | change_bit;
            if new_mask.count_ones() as i32 > k {
                ret = 1 + solve(idx + 1, change_bit, has_changed, k, s, memo);
            } else {
                ret = solve(idx + 1, new_mask, has_changed, k, s, memo);
            }
            // 2. change current char with guard
            if !has_changed {
                for c in 0..26 {
                    let change_bit = 1 << c as i32;
                    let new_mask = mask | change_bit;
                    if new_mask.count_ones() as i32 > k {
                        ret = ret.max(1 + solve(idx + 1, change_bit, true, k, s, memo));
                    } else {
                        ret = ret.max(solve(idx + 1, new_mask, true, k, s, memo));
                    }
                }
            }
            memo.insert(state, ret);

            ret
        }

        solve(0, 0, false, k, &bytes, &mut memo)
    }
}

fn main() {}
