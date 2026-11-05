#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let product = nums1[i] * nums2[j];
                if i > 0 && j > 0 {
                    // set a default
                    dp[i][j] = product + dp[i - 1][j - 1].max(0);
                } else {
                    dp[i][j] = product;
                }
                if i > 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[m - 1][n - 1]
    }
}

fn main() {}
