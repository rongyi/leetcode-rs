struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut suffix_sum = vec![0; sz + 1];
        for i in (0..sz).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + nums[i];
        }
        let mut left_side_sum = 0;
        for i in 0..sz {
            if left_side_sum == suffix_sum[i + 1] {
                return i as i32;
            }
            // after check, not include cur
            left_side_sum += nums[i];
        }
        -1
    }
}

fn main() {}
