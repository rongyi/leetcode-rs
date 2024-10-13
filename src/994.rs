struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut num_good = 0;
        let mut q = VecDeque::new();
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    num_good += 1;
                }
                if grid[i][j] == 2 {
                    q.push_back((i as i32, j as i32));
                }
            }
        }
        if num_good == 0 {
            return 0;
        }
        let mut ret = -1;
        while !q.is_empty() {
            ret += 1;
            let sz = q.len();
            for _ in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                // rotten it
                grid[x as usize][y as usize] = 2;

                for &[dx, dy] in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0
                        || nx >= m as i32
                        || ny < 0
                        || ny >= n as i32
                        || grid[nx as usize][ny as usize] != 1
                    {
                        continue;
                    }
                    grid[nx as usize][ny as usize] = 2;
                    num_good -= 1;
                    q.push_back((nx, ny));
                }
            }
        }
        if num_good > 0 {
            return -1;
        }

        ret
    }
}

fn main() {}
