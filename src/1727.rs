#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        // Pre-process the matrix to count consecutive 1s from top to bottom
        let mut heights = vec![vec![0; cols]; rows];

        // Initialize the first row
        for j in 0..cols {
            heights[0][j] = matrix[0][j];
        }

        // Calculate heights for the rest of the rows
        for i in 1..rows {
            for j in 0..cols {
                if matrix[i][j] == 1 {
                    heights[i][j] = heights[i - 1][j] + 1;
                } else {
                    heights[i][j] = 0;
                }
            }
        }

        let mut max_area = 0;

        // For each row, sort the heights and calculate the maximum area
        for i in 0..rows {
            let mut row_heights = heights[i].clone();
            row_heights.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

            for (j, &height) in row_heights.iter().enumerate() {
                if height == 0 {
                    break;
                }
                let width = j + 1;
                max_area = max_area.max(height * width as i32);
            }
        }

        max_area
    }
}
fn main() {}
