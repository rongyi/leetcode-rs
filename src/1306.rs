#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visted: Vec<bool> = vec![false; arr.len()];
        Self::dfs(&arr, start, &mut visted)
    }
    fn dfs(arr: &Vec<i32>, start: i32, visited: &mut Vec<bool>) -> bool {
        if arr[start as usize] == 0 {
            return true;
        }
        visited[start as usize] = true;
        // jump backward
        let idx = (start + arr[start as usize]) as usize;
        if idx < arr.len() && !visited[idx] {
            if Self::dfs(arr, idx as i32, visited) {
                return true;
            }
        }

        // jump forward
        let idx = start - arr[start as usize];
        if idx >= 0 && !visited[idx as usize] {
            if Self::dfs(arr, idx, visited) {
                return true;
            }
        }

        false
    }
}

fn main() {}
