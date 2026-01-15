struct Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut max_sum = *nums.last().unwrap() as i64;

        for i in (0..nums.len() - 1).rev() {
            let cur = nums[i] as i64;
            if cur <= max_sum {
                // eat it
                max_sum += cur;
            } else {
                max_sum = cur;
            }
        }
        max_sum
    }
}

fn main() {}
