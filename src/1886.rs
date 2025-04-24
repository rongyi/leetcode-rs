#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();

        // Check if matrices are identical
        let matches_original = (0..n).all(|i| (0..n).all(|j| mat[i][j] == target[i][j]));
        if matches_original {
            return true;
        }

        // Check 90 degrees rotation
        let matches_90 = (0..n).all(|i| (0..n).all(|j| mat[i][j] == target[j][n - 1 - i]));
        if matches_90 {
            return true;
        }

        // Check 180 degrees rotation
        let matches_180 = (0..n).all(|i| (0..n).all(|j| mat[i][j] == target[n - 1 - i][n - 1 - j]));
        if matches_180 {
            return true;
        }

        // Check 270 degrees rotation
        let matches_270 = (0..n).all(|i| (0..n).all(|j| mat[i][j] == target[n - 1 - j][i]));
        if matches_270 {
            return true;
        }

        false
    }
}

fn main() {}
