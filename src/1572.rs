#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut sum = 0;

        for i in 0..n {
            // Primary diagonal (top-left to bottom-right)
            sum += mat[i][i];

            // Secondary diagonal (top-right to bottom-left)
            let j = n - 1 - i;

            // Avoid counting the center element twice in odd-sized matrices
            if i != j {
                sum += mat[i][j];
            }
        }

        sum
    }
}

fn main() {}
