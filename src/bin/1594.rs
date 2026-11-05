#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let modulo: i64 = 1_000_000_007;

        // Create two DP arrays to track maximum and minimum products
        // We need to track both because negative * negative = positive
        let mut max_product = vec![vec![0_i64; cols]; rows];
        let mut min_product = vec![vec![0_i64; cols]; rows];

        // Initialize the first cell
        max_product[0][0] = grid[0][0] as i64;
        min_product[0][0] = grid[0][0] as i64;

        // Fill the first row
        for j in 1..cols {
            max_product[0][j] = max_product[0][j - 1] * (grid[0][j] as i64);
            min_product[0][j] = min_product[0][j - 1] * (grid[0][j] as i64);
        }

        // Fill the first column
        for i in 1..rows {
            max_product[i][0] = max_product[i - 1][0] * (grid[i][0] as i64);
            min_product[i][0] = min_product[i - 1][0] * (grid[i][0] as i64);
        }

        // Fill the rest of the matrix
        for i in 1..rows {
            for j in 1..cols {
                let current_value = grid[i][j] as i64;

                if current_value >= 0 {
                    // If current cell value is non-negative
                    max_product[i][j] =
                        current_value * max_product[i][j - 1].max(max_product[i - 1][j]);
                    min_product[i][j] =
                        current_value * min_product[i][j - 1].min(min_product[i - 1][j]);
                } else {
                    // If current cell value is negative
                    max_product[i][j] =
                        current_value * min_product[i][j - 1].min(min_product[i - 1][j]);
                    min_product[i][j] =
                        current_value * max_product[i][j - 1].max(max_product[i - 1][j]);
                }
            }
        }

        // Check if the maximum product at the bottom-right is negative
        if max_product[rows - 1][cols - 1] < 0 {
            return -1;
        }

        // Return the maximum product modulo 10^9 + 7
        (max_product[rows - 1][cols - 1] % modulo) as i32
    }
}

fn main() {}
