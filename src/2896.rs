struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let sz = s1.len();
        let diffs: Vec<usize> = (0..sz).filter(|&i| s1[i] != s2[i]).collect();
        if diffs.is_empty() {
            return 0;
        }
        if diffs.len() % 2 != 0 {
            return -1;
        }
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();

        fn solve(
            idx: usize,
            pending: usize,
            diffs: &Vec<usize>,
            x: i32,
            sz: usize,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if idx == sz {
                return if pending == 0 { 0 } else { 1_000_000_000 };
            }
            if let Some(&cache) = memo.get(&(idx, pending)) {
                return cache;
            }
            let val1 = solve(idx + 1, 1, diffs, x, sz, memo) + x;
            let mut ret = val1;
            if idx + 1 < sz {
                let cost_adj = (diffs[idx + 1] - diffs[idx]) as i32;
                ret = ret.min(solve(idx + 2, pending, diffs, x, sz, memo) + cost_adj);
            }
            if pending > 0 {
                ret = ret.min(solve(idx + 1, pending - 1, diffs, x, sz, memo));
            }
            memo.insert((idx, pending), ret);

            ret
        }

        solve(0, 0, &diffs, x, diffs.len(), &mut memo)
    }
}

fn main() {}
