#![allow(dead_code)]
struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let init = "0000".to_string();
        let mut visited = HashSet::new();
        let dds: HashSet<_> = deadends.into_iter().collect();
        let mut q: VecDeque<String> = VecDeque::new();

        // shit, can not even move a single step
        if dds.contains(&init) {
            return -1;
        }
        if target == init {
            return 0;
        }
        visited.insert(init.clone());
        q.push_back(init);

        let mut ret = 0;

        // bfs
        while !q.is_empty() {
            let sz = q.len();
            // a layer
            for _ in 0..sz {
                let cur = q.pop_front().unwrap();
                let neis = Self::neighbors(&cur);

                for s in neis.into_iter() {
                    if s == target {
                        return ret + 1;
                    }
                    if visited.contains(&s) {
                        continue;
                    }
                    if !dds.contains(&s) {
                        q.push_back(s.clone());
                        visited.insert(s);
                    }
                }
            }
            ret += 1;
        }

        -1
    }

    fn neighbors(cur: &str) -> Vec<String> {
        let cur: Vec<_> = cur.chars().collect();

        let mut ret: Vec<String> = Vec::new();
        for i in 0..cur.len() {
            let mut tmp = cur.clone();
            tmp[i] = ((cur[i] as u8 - '0' as u8 + 1) % 10 + '0' as u8) as char;
            ret.push(tmp.into_iter().collect());

            let mut tmp = cur.clone();
            tmp[i] = ((cur[i] as u8 - '0' as u8 + 9) % 10 + '0' as u8) as char;
            ret.push(tmp.into_iter().collect());
        }

        ret
    }
}

fn main() {}
