
struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut trees = Vec::new();
        for i in 0..forest.len() {
            for j in 0..forest[0].len() {
                if forest[i][j] > 1 {
                    trees.push((forest[i][j], i as i32, j as i32));
                }
            }
        }
        trees.sort_unstable();

        let mut ret = 0;

        let mut cur_row = 0;
        let mut cur_col = 0;
        for i in 0..trees.len() {
            let step = Self::next_step(&forest, cur_row, cur_col, trees[i].1, trees[i].2);
            if step == -1 {
                return -1;
            }
            ret += step;
            cur_row = trees[i].1;
            cur_col = trees[i].2;
        }

        ret
    }
    fn next_step(forest: &Vec<Vec<i32>>, sx: i32, sy: i32, ex: i32, ey: i32) -> i32 {
        if sx == ex && sy == ey {
            return 0;
        }
        let mut q = VecDeque::new();
        q.push_back((sx, sy));

        let m = forest.len();
        let n = forest[0].len();
        let mut visited = vec![vec![false; n]; m];
        visited[sx as usize][sy as usize] = true;

        let mut step = 0;
        while !q.is_empty() {
            step += 1;
            let sz = q.len();
            for _ in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                for d in &[[-1, 0], [1, 0], [0, 1], [0, -1]] {
                    let nx = x + d[0];
                    let ny = y + d[1];
                    if nx < 0
                        || nx >= m as i32
                        || ny < 0
                        || ny >= n as i32
                        || visited[nx as usize][ny as usize]
                        || forest[nx as usize][ny as usize] == 0
                    {
                        continue;
                    }
                    if nx == ex && ny == ey {
                        return step;
                    }
                    visited[nx as usize][ny as usize] = true;
                    q.push_back((nx, ny));
                }
            }
        }

        -1
    }
}

fn main() {}
