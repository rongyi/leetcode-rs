#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let s = s.as_bytes();
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;
        let n = s.len();

        if s[n - 1] == b'1' {
            return false;
        }

        let mut dp = vec![false; n];
        dp[0] = true;

        // Use a sliding window to track the sum of dp[i] within the valid jump range
        let mut window_count = 0;

        for i in 1..n {
            // Update the window: add elements that came into range
            if i >= min_jump && dp[i - min_jump] {
                window_count += 1;
            }

            // Remove elements that went out of range
            if i > max_jump && dp[i - max_jump - 1] {
                window_count -= 1;
            }

            // Update dp[i] if there's a valid jump position in the window and current position is '0'
            dp[i] = window_count > 0 && s[i] == b'0';
        }

        dp[n - 1]
    }
}
fn main() {}
