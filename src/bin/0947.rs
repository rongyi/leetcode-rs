
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut island = 0;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        for s in stones.iter() {
            let (x, y) = (s[0], s[1]);
            if !visited.contains(&(x, y)) {
                island += 1;
                Self::dfs(&stones, &mut visited, x, y);
            }
        }

        stones.len() as i32 - island
    }

    fn dfs(stones: &Vec<Vec<i32>>, visited: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
        visited.insert((x, y));
        for s in stones.iter() {
            let nx = s[0];
            let ny = s[1];
            if !visited.contains(&(nx, ny)) && ((x == nx) || (y == ny)) {
                Self::dfs(stones, visited, nx, ny);
            }
        }
    }
}

fn main() {}
