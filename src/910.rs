struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let left = nums[0] + k;
        let right = nums.last().unwrap() - k;
        let mut ret = nums.last().unwrap() - nums.first().unwrap();
        for i in 0..nums.len() - 1 {
            let cur_max = right.max(nums[i] + k);
            let cur_min = left.min(nums[i + 1] - k);
            ret = ret.min(cur_max - cur_min);
        }

        ret
    }
}

fn main() {}
