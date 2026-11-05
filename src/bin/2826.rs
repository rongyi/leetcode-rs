struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut dp = vec![0; 3];
        for &num in nums.iter() {
            let val = (num - 1) as usize;

            let mut max_prev = 0;
            for j in 0..=val {
                max_prev = max_prev.max(dp[j]);
            }
            dp[val] = max_prev + 1;
        }
        sz as i32 - dp.into_iter().max().unwrap()
    }
}

fn main() {}
