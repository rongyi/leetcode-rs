
struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    mat[i][j] = i32::MAX;
                } else {
                    q.push_back((i as i32, j as i32));
                }
            }
        }
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            for d in dirs.iter() {
                let nx = x + d[0];
                let ny = y + d[1];
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                if mat[nx as usize][ny as usize] > mat[x as usize][y as usize] + 1 {
                    mat[nx as usize][ny as usize] = mat[x as usize][y as usize] + 1;
                    q.push_back((nx, ny));
                }
            }
        }
        mat
    }
}

fn main() {}
