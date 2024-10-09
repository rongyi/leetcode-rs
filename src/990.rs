
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut equals: HashMap<char, HashSet<char>> = HashMap::new();
        let mut paradox: Vec<(char, char)> = Vec::new();

        for s in equations.into_iter() {
            let s: Vec<char> = s.chars().collect();
            let c1 = s[0];
            let c2 = s[3];
            if s[1] == '!' {
                // imediate case
                if c1 == c2 {
                    return false;
                }
                paradox.push((c1, c2));
            } else {
                equals.entry(c1).or_insert(HashSet::new()).insert(c2);
                equals.entry(c2).or_insert(HashSet::new()).insert(c1);
            }
        }

        for (c1, c2) in paradox.into_iter() {
            let mut c1_family = HashSet::new();
            Self::dfs(&equals, &mut c1_family, c1);
            let mut c2_family = HashSet::new();
            Self::dfs(&equals, &mut c2_family, c2);
            // 两边是水火不容的
            for l in c1_family.into_iter() {
                if c2_family.contains(&l) {
                    return false;
                }
            }
        }
        true
    }

    fn dfs(equals: &HashMap<char, HashSet<char>>, visited: &mut HashSet<char>, cur: char) {
        if visited.contains(&cur) {
            return;
        }
        visited.insert(cur);
        if let Some(e) = equals.get(&cur) {
            for &nc in e.iter() {
                Self::dfs(equals, visited, nc);
            }
        }
    }
}

fn main() {}
