struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut cur = Vec::new();
        let mut seen: HashSet<Vec<i32>> = HashSet::new();

        Self::backtrack(&nums, 0, &mut cur, &mut ret, &mut seen);
        ret
    }

    fn backtrack(
        nums: &[i32],
        idx: usize,
        cur_seq: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
        seen: &mut HashSet<Vec<i32>>,
    ) {
        if cur_seq.len() > 1 && !seen.contains(cur_seq) {
            ret.push(cur_seq.clone());
            seen.insert(cur_seq.clone());
        }
        for i in idx..nums.len() {
            if cur_seq.is_empty() || nums[i] >= *cur_seq.last().unwrap() {
                cur_seq.push(nums[i]);
                Self::backtrack(nums, i + 1, cur_seq, ret, seen);
                cur_seq.pop();
            }
        }
    }
}

fn main() {}
