struct Solution;


impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp= vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &v in nums.iter() {
                if i - v >= 0 {
                    dp[i as usize] += dp[(i - v) as usize];
                }
            }
        }
        dp[target as usize]
    }
}


fn main() {}
