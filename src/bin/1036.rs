
struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let blocked: HashSet<(i32, i32)> = blocked.into_iter().map(|p| (p[0], p[1])).collect();
        let max_area = (blocked.len() * (blocked.len() - 1)) / 2;

        Self::can_reach(&source, &target, &blocked, max_area)
            && Self::can_reach(&target, &source, &blocked, max_area)
    }

    fn can_reach(
        start: &Vec<i32>,
        end: &Vec<i32>,
        blocked: &HashSet<(i32, i32)>,
        max_area: usize,
    ) -> bool {
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((start[0], start[1]));
        visited.insert((start[0], start[1]));
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((x, y)) = q.pop_front() {
            if visited.len() > max_area || (x == end[0] && y == end[1]) {
                return true;
            }
            for &(dx, dy) in dirs.iter() {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0
                    || nx >= 1_000_000
                    || ny < 0
                    || ny >= 1_000_000
                    || blocked.contains(&(nx, ny))
                    || visited.contains(&(nx, ny))
                {
                    continue;
                }
                visited.insert((nx, ny));
                q.push_back((nx, ny));
            }
        }
        false
    }
}

fn main() {}
