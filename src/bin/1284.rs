#![allow(dead_code)]

struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        // (state, change_acc)
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();
        // at most 3 * 3, so we can transform matrix to a int
        let mut init_state = 0;
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    init_state |= 1 << (i * n + j);
                }
            }
        }
        // perfect
        if init_state == 0 {
            return 0;
        }
        q.push_back((init_state, 0));
        visited.insert(init_state);
        // note the origin point is contained
        let dirs = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some((cur_stat, ch_acc)) = q.pop_front() {
            for i in 0..m {
                for j in 0..n {
                    let mut next_stat = cur_stat;

                    for &(dx, dy) in dirs.iter() {
                        let nx = i as i32 + dx;
                        let ny = j as i32 + dy;
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                            next_stat ^= 1 << (nx as usize * n + ny as usize);
                        }
                    }

                    if next_stat == 0 {
                        return ch_acc + 1;
                    }

                    if !visited.contains(&next_stat) {
                        visited.insert(next_stat);
                        q.push_back((next_stat, ch_acc + 1));
                    }
                }
            }
        }

        -1
    }
}
fn main() {}
