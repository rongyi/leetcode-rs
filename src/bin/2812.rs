struct Solution;

use std::collections::{BinaryHeap, VecDeque};
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        // sz x sz
        let sz = grid.len();
        let mut dist = vec![vec![i32::MAX; sz]; sz];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        for i in 0..sz {
            for j in 0..sz {
                // start from thief point
                if grid[i][j] == 1 {
                    dist[i][j] = 0;
                    q.push_back((i, j));
                }
            }
        }
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((x, y)) = q.pop_front() {
            for d in dirs.iter() {
                let nx = x as i32 + d.0;
                let ny = y as i32 + d.1;
                if nx < 0 || nx >= sz as i32 || ny < 0 || ny >= sz as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if dist[nx][ny] == i32::MAX {
                    dist[nx][ny] = dist[x][y] + 1;
                    q.push_back((nx, ny));
                }
            }
        }
        // now we get dist graph
        // The safeness factor of a path on the grid is defined as the minimum manhattan distance
        // from any cell in the path to any thief in the grid.
        // all the path, return those max value
        let mut path_safe = vec![vec![-1; sz]; sz];
        path_safe[0][0] = dist[0][0];
        // ()
        let mut q: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        q.push((dist[0][0], 0, 0));
        while let Some((cur_safe, x, y)) = q.pop() {
            if x == sz - 1 && y == sz - 1 {
                return cur_safe;
            }
            for d in dirs.iter() {
                let nx = x as i32 + d.0;
                let ny = y as i32 + d.1;
                if nx < 0 || nx >= sz as i32 || ny < 0 || ny >= sz as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                let new_safe = cur_safe.min(dist[nx][ny]);
                if new_safe > path_safe[nx][ny] {
                    path_safe[nx][ny] = new_safe;
                    q.push((new_safe, nx, ny));
                }
            }
        }

        0
    }
}

fn main() {}
