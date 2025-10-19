struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut left_sum = vec![0; sz];
        for i in 1..sz {
            left_sum[i] = left_sum[i - 1] + nums[i - 1];
        }
        let mut right_sum = vec![0; sz];
        for i in (0..sz - 1).rev() {
            right_sum[i] = right_sum[i + 1] + nums[i + 1];
        }
        left_sum
            .into_iter()
            .zip(right_sum.into_iter())
            .map(|(a, b)| (a - b).abs())
            .collect()
    }
}

fn main() {}
