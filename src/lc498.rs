struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut visited = vec![vec![false; n]; m];
        let dirs = [[0, 1], [1, 0]];
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((0, 0));
        visited[0][0] = true;
        let mut ret: Vec<Vec<i32>> = Vec::new();

        while !q.is_empty() {
            let mut cur_layer = Vec::new();
            let sz = q.len();
            for _i in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                cur_layer.push(mat[x as usize][y as usize]);
                for d in &dirs {
                    let (nx, ny) = (x + d[0], y + d[1]);
                    if nx < 0
                        || nx >= m as i32
                        || ny < 0
                        || ny >= n as i32
                        || visited[nx as usize][ny as usize]
                    {
                        continue;
                    }
                    visited[nx as usize][ny as usize] = true;
                    q.push_back((nx, ny));
                }
            }
            ret.push(cur_layer);
        }
        let mut flat = Vec::new();
        for cur in ret.iter_mut().step_by(2) {
            cur.reverse();
        }
        for cur in ret.iter() {
            flat.extend(cur);
        }
        flat
    }
}

fn main() {
    let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::find_diagonal_order(input);
}
