struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut start = 0;
        let mut min_idx: Option<usize> = None;
        let mut max_idx: Option<usize> = None;
        let mut ret = 0;

        for i in 0..nums.len() {
            if nums[i] < min_k || nums[i] > max_k {
                min_idx = None;
                max_idx = None;
                start = i + 1;
            }
            if nums[i] == min_k {
                min_idx = Some(i);
            }
            if nums[i] == max_k {
                max_idx = Some(i);
            }
            if min_idx.is_some() && max_idx.is_some() {
                let min = min_idx.unwrap();
                let max = max_idx.unwrap();
                ret += (min.min(max) - start + 1) as i64;
            }
        }

        ret
    }
}

fn main() {}
