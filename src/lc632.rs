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
        let mut ret: Vec<i32> = Vec::new();
        let mut cnt: HashMap<usize, i32> = HashMap::new();
        // sliding window
        let mut i = 0;
        let mut k = 0;
        for j in 0..ordered.len() {
            if !cnt.contains_key(&ordered[j].1) || cnt[&ordered[j].1] == 0 {
                k += 1;
            }
            *cnt.entry(ordered[j].1).or_insert(0) += 1;
            // try shrink
            if k == nums.len() {
                while cnt[&ordered[i].1] > 1 {
                    *cnt.get_mut(&ordered[i].1).unwrap() -= 1;
                    i += 1;
                    println!("loop here");
                }
                if ret.is_empty() || ret[1] - ret[0] > ordered[j].0 - ordered[i].0 {
                    ret = vec![ordered[i].0, ordered[j].0];
                }
            }
        }

        ret
    }
}

fn main() {}
