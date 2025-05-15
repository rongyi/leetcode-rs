#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let rows = points.len();
        let cols = points[0].len();
        let mut dp = vec![vec![0i64; cols]; rows];

        // Initialize first row
        for (j, &val) in points[0].iter().enumerate() {
            dp[0][j] = val as i64;
        }

        // Process each subsequent row
        for i in 1..rows {
            let mut left_to_right = dp[i - 1].clone();
            let mut right_to_left = dp[i - 1].clone();

            // Calculate left to right maximum values considering cost
            for j in 1..cols {
                left_to_right[j] = left_to_right[j].max(left_to_right[j - 1] - 1);
            }

            // Calculate right to left maximum values considering cost
            for j in (0..cols - 1).rev() {
                right_to_left[j] = right_to_left[j].max(right_to_left[j + 1] - 1);
            }

            // Calculate dp values for current row
            for j in 0..cols {
                dp[i][j] = points[i][j] as i64 + left_to_right[j].max(right_to_left[j]);
            }
        }

        // Return maximum value from last row
        *dp[rows - 1].iter().max().unwrap()
    }
}

fn main() {}
