struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut uniq: HashSet<String> = HashSet::new();

        let mut cur = String::new();

        let tiles: Vec<char> = tiles.chars().collect();
        let mut visited = vec![false; tiles.len()];
        Self::dfs(&tiles, &mut visited, &mut uniq, &mut cur);

        println!("{:?}", uniq);

        uniq.len() as i32
    }

    fn dfs(input: &[char], visited: &mut Vec<bool>, uniq: &mut HashSet<String>, cur: &mut String) {
        if visited.iter().all(|&x| x) {
            if !cur.is_empty() {
                uniq.insert(cur.clone());
            }
            return;
        }

        for i in 0..input.len() {
            if visited[i] {
                continue;
            }

            // 1. don't take current
            visited[i] = true;
            Self::dfs(input, visited, uniq, cur);

            // 2. take current
            cur.push(input[i]);
            Self::dfs(input, visited, uniq, cur);
            visited[i] = false;
            cur.pop();
        }
    }
}

fn main() {}
