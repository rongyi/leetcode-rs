#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        let mut cuboids = cuboids;

        // Sort each cuboid dimensions in non-decreasing order
        for cuboid in cuboids.iter_mut() {
            cuboid.sort();
        }

        // Sort cuboids by their dimensions in non-decreasing order
        cuboids.sort_unstable_by(|a, b| a.cmp(&b));

        let n = cuboids.len();
        let mut dp = vec![0; n];

        // Base case: each cuboid can be a stack by itself
        for i in 0..n {
            dp[i] = cuboids[i][2]; // Height of cuboid i
        }

        // Dynamic programming to find the maximum height
        // just like subsequence!
        for i in 0..n {
            for j in 0..i {
                // Check if cuboid i can be stacked on top of cuboid j
                if cuboids[j][0] <= cuboids[i][0]
                    && cuboids[j][1] <= cuboids[i][1]
                    && cuboids[j][2] <= cuboids[i][2]
                {
                    dp[i] = dp[i].max(dp[j] + cuboids[i][2]);
                }
            }
        }

        *dp.iter().max().unwrap()
    }
}

fn main() {}
