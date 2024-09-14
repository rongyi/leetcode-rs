struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        let diff = max - min - 2 * k;
        if diff > 0 {
            diff
        } else {
            0
        }
    }
}

fn main() {}
