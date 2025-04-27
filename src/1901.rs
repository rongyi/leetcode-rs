#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = mat.len();
        let cols = mat[0].len();

        let mut start_col = 0;
        let mut end_col = cols - 1;

        while start_col <= end_col {
            let mid_col = start_col + (end_col - start_col) / 2;

            // Find the row with the maximum value in the current column
            let mut max_row = 0;
            for row in 1..rows {
                if mat[row][mid_col] > mat[max_row][mid_col] {
                    max_row = row;
                }
            }

            // Check if neighbors to the left and right are smaller
            let is_left_smaller = mid_col == 0 || mat[max_row][mid_col] > mat[max_row][mid_col - 1];
            let is_right_smaller =
                mid_col == cols - 1 || mat[max_row][mid_col] > mat[max_row][mid_col + 1];

            if is_left_smaller && is_right_smaller {
                // Found a peak element
                return vec![max_row as i32, mid_col as i32];
            } else if !is_left_smaller {
                // Move to the left
                end_col = mid_col - 1;
            } else {
                // Move to the right
                start_col = mid_col + 1;
            }
        }

        // The problem guarantees a peak exists
        vec![-1, -1]
    }
}

fn main() {}
