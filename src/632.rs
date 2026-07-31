struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ordered: Vec<(i32, usize)> = Vec::new();
        for (i, lst) in nums.iter().enumerate() {
            for &num in lst.iter() {
                ordered.push((num, i));
            }
        }
        ordered.sort_unstable();
        let mut ret: Vec<i32> = vec![0, i32::MAX];
        let mut cnt: HashMap<usize, i32> = HashMap::new();
        // sliding window
        let mut i = 0;
        let mut k = 0;
        for j in 0..ordered.len() {
            let (right_val, right_idx) = ordered[j];
            let e = cnt.entry(right_idx).or_default();
            if *e == 0 {
                k += 1;
            }
            *e += 1;

            while k == nums.len() {
                let left_val = ordered[i].0;
                if right_val - left_val < ret[1] - ret[0] {
                    ret[0] = left_val;
                    ret[1] = right_val;
                }
                let left_idx = ordered[i].1;
                if let Some(c) = cnt.get_mut(&left_idx) {
                    *c -= 1;
                    if *c == 0 {
                        k -= 1;
                    }
                }
                i += 1;
            }
        }

        ret
    }
}

fn main() {}
