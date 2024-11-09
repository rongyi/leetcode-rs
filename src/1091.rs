struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((0, 0));
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        while !q.is_empty() {
            ret += 1;
            let mut q2: VecDeque<(i32, i32)> = VecDeque::new();
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                if grid[x as usize][y as usize] == 1 {
                    continue;
                }
                grid[x as usize][y as usize] = 1;

                if x == m - 1 && y == n - 1 {
                    return ret;
                }
                // 八个方向
                // .  .  .
                //    |
                // .__|__.
                //    |
                // .  .  .
                for nx in x - 1..=x + 1 {
                    for ny in y - 1..=y + 1 {
                        if nx != x || ny != y {
                            if nx >= 0
                                && nx < m
                                && ny >= 0
                                && ny < n
                                && grid[nx as usize][ny as usize] == 0
                            {
                                q2.push_back((nx, ny));
                            }
                        }
                    }
                }
            }
            q = q2.clone();
            q2.clear();
        }
        -1
    }
}

fn main() {}
