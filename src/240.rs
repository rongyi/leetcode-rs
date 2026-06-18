struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut i = 0;
        let mut j = n as i32 - 1;
        while i < m && j >= 0 {
            if matrix[i][j as usize] == target {
                return true;
            } else if matrix[i][j as usize] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        false
    }
}

fn main() {}
