
struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        // change to 1d
        let mut start_state = [0; 9];
        for i in 0..3 {
            for j in 0..3 {
                start_state[i * 3 + j] = grid[i][j];
            }
        }
        let mut target = [1; 9];
        let mut visited: HashSet<[i32; 9]> = HashSet::new();
        visited.insert(start_state);
        let mut q: VecDeque<([i32; 9], i32)> = VecDeque::new();
        q.push_back((start_state, 0));
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((cur_state, cur_step)) = q.pop_front() {
            if cur_state == target {
                return cur_step;
            }
            for i in 0..9 {
                if cur_state[i] > 1 {
                    let x = i as i32 / 3;
                    let y = i as i32 % 3;

                    for &(dx, dy) in dirs.iter() {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx < 0 || nx >= 3 || ny < 0 || ny >= 3 {
                            continue;
                        }
                        let nx = nx as usize;
                        let ny = ny as usize;
                        let flat_idx = nx * 3 + ny;
                        let mut next_state = cur_state;
                        next_state[i] -= 1;
                        next_state[flat_idx] += 1;
                        if !visited.contains(&next_state) {
                            visited.insert(next_state);
                            q.push_back((next_state, cur_step + 1));
                        }
                    }
                }
            }
        }

        0
    }
}

fn main() {}
