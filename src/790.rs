#![allow(dead_code)]
struct Solution;

impl Solution {
    // https://leetcode.com/problems/domino-and-tromino-tiling/solutions/1620809/python-java-c-c-dp-image-visualized-explanation-100-faster-o-n/
    pub fn num_tilings(n: i32) -> i32 {
        let m = 1e9 as i32 + 7;
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            let i = i as usize;
            dp[i] = (dp[i - 1] * 2 + dp[i - 3]) % m;
        }
        
        dp[n as usize]
    }
}

fn main() {}
