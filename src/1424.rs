#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn find_diagonal_order(mut nums: Vec<Vec<i32>>) -> Vec<i32> {
        // using bfs?
        let m = nums.len();
        // order matters!
        let dirs = [(1, 0), (0, 1)];

        let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
        q.push_back((0, 0, nums[0][0]));

        let mut ret: Vec<i32> = Vec::new();
        while let Some((x, y, val)) = q.pop_front() {
            ret.push(val);
            for &(dx, dy) in dirs.iter() {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && nx < m as i32
                    && ny >= 0
                    && ny < nums[nx as usize].len() as i32
                    && nums[nx as usize][ny as usize] > 0
                {
                    q.push_back((nx, ny, nums[nx as usize][ny as usize]));
                    nums[nx as usize][ny as usize] = 0;
                }
            }
        }

        ret
    }
}

fn main() {}
