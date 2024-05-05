#![allow(dead_code)]


struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut visited = vec![vec![false; n]; m];
        visited[0][n - 1] = true;
        let mut q = VecDeque::new();
        q.push_back((0, n - 1));

        while !q.is_empty() {
            let sz = q.len();
            let mut val = None;
            for _ in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                match val {
                    Some(val) => {
                        if val != matrix[x][y] {
                            return false;
                        }
                    }
                    None => val = Some(matrix[x][y]),
                }
                for d in &[[0, 1], [1, 0], [-1, 0], [0, -1]] {
                    let nx = x as i32 + d[0];
                    let ny = y as i32 + d[1];
                    if nx < 0
                        || nx >= m as i32
                        || ny < 0
                        || ny >= n as i32
                        || visited[nx as usize][ny as usize]
                    {
                        continue;
                    }
                    visited[nx as usize][ny as usize] = true;

                    q.push_back((nx as usize, ny as usize));
                }
            }
        }

        true
    }
}

fn main() {}
