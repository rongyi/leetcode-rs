struct Solution;

mod ai {
    struct Solution;
    struct Dsu {
        parent: [usize; 26],
    }

    impl Dsu {
        fn new() -> Self {
            let mut parent = [0; 26];
            for i in 0..26 {
                parent[i] = i;
            }
            Self { parent }
        }

        fn find(&mut self, i: usize) -> usize {
            if self.parent[i] == i {
                i
            } else {
                self.parent[i] = self.find(self.parent[i]);
                self.parent[i]
            }
        }

        fn union(&mut self, i: usize, j: usize) {
            let root_i = self.find(i);
            let root_j = self.find(j);
            if root_i != root_j {
                self.parent[root_i] = root_j;
            }
        }
    }

    impl Solution {
        pub fn equations_possible(equations: Vec<String>) -> bool {
            let mut dsu = Dsu::new();

            // Pass 1: Union all variables connected by '=='
            for eq in &equations {
                let bytes = eq.as_bytes();
                if bytes[1] == b'=' {
                    let u = (bytes[0] - b'a') as usize;
                    let v = (bytes[3] - b'a') as usize;
                    dsu.union(u, v);
                }
            }

            // Pass 2: Check for contradictions in '!=' equations
            for eq in &equations {
                let bytes = eq.as_bytes();
                if bytes[1] == b'!' {
                    let u = (bytes[0] - b'a') as usize;
                    let v = (bytes[3] - b'a') as usize;
                    if dsu.find(u) == dsu.find(v) {
                        return false;
                    }
                }
            }

            true
        }
    }
}

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
