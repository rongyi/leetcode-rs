#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let bottom: Vec<char> = bottom.chars().collect();
        let allowed: Vec<Vec<char>> = allowed.into_iter().map(|s| s.chars().collect()).collect();
        let mut tris: HashMap<Vec<char>, Vec<char>> = HashMap::new();

        for s in allowed.iter() {
            let mut s = s.clone();
            let last = s.pop().unwrap();
            tris.entry(s).or_insert(Vec::new()).push(last);
        }

        let mut next = Vec::new();
        Self::dfs(bottom, &tris, 0, &mut next)
    }

    fn dfs(
        bottom: Vec<char>,
        tris: &HashMap<Vec<char>, Vec<char>>,
        start: usize,
        next: &mut Vec<char>,
    ) -> bool {
        if bottom.len() == 1 {
            return true;
        }

        if start == bottom.len() - 1 {
            return Self::dfs(next.clone(), tris, 0, &mut vec![]);
        }

        let key = bottom[start..start + 2]
            .iter()
            .map(|c| *c)
            .collect::<Vec<char>>();
        for &c in tris.get(&key).unwrap_or(&vec![]).iter() {
            next.push(c);
            if Self::dfs(bottom.clone(), tris, start + 1, next) {
                return true;
            }
            next.pop();
        }

        false
    }
}

fn main() {}
