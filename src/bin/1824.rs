#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len();
        // dp[lane] = minimum jumps to reach current position in this lane
        // The dp array tracks minimum jumps required for each lane
        // Initially, frog starts at lane 2 (index 2), so 0 jumps needed there
        // Lanes 1 and 3 require 1 sideway jump to reach from starting position
        let mut dp = [0, 1, 0, 1]; // 1-indexed lanes (0 is a dummy)

        for i in 1..n {
            let obstacle = obstacles[i];

            // Set the lane with obstacle to a large value
            if obstacle > 0 {
                dp[obstacle as usize] = i32::MAX - 1;
            }

            // Update each lane with the best possible value
            for lane in 1..=3 {
                if lane as i32 != obstacle {
                    dp[lane] = dp[lane].min(dp[1].min(dp[2]).min(dp[3]) + 1).min(dp[lane]);
                }
            }
        }

        // Return minimum jumps among all lanes at the end
        dp[1].min(dp[2]).min(dp[3])
    }
}

fn main() {}
