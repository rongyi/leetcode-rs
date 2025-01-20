#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut itvs = Vec::new();
        for (i, rg) in ranges.into_iter().enumerate() {
            itvs.push((i as i32 - rg, i as i32 + rg));
        }
        let mut i = 0;
        let mut start = 0;
        let mut end = 0;
        let mut ret = 0;

        itvs.sort_unstable();

        while start < n {
            // can be together
            while i < itvs.len() as i32 && itvs[i as usize].0 <= start {
                end = end.max(itvs[i as usize].1);
                i += 1;
            }
            if start == end {
                return -1;
            }

            ret += 1;
            start = end;
        }

        ret
    }
}

fn main() {}
