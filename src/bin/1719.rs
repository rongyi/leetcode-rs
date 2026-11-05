#![allow(dead_code)]

struct Solution;

use std::collections::{BinaryHeap, HashMap, HashSet};
impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();

        for p in pairs.iter() {
            adj.entry(p[0]).or_default().insert(p[1]);
            adj.entry(p[1]).or_default().insert(p[0]);
        }
        let mut q: BinaryHeap<(usize, i32)> = BinaryHeap::new();
        for (&k, v) in adj.iter() {
            q.push((v.len(), k));
        }
        let n = q.len();
        let mut multiple = false;
        let mut visited: HashSet<i32> = HashSet::new();

        while !q.is_empty() {
            let (cur_degress, cur) = q.pop().unwrap();
            let mut root = 0;
            let mut root_degree = n + 1;

            if !visited.is_empty() {
                for &x in adj[&cur].iter() {
                    if visited.contains(&x) && adj[&x].len() < root_degree {
                        root = x;
                        root_degree = adj[&x].len();
                    }
                }
            }

            visited.insert(cur);
            if root == 0 {
                if cur_degress != n - 1 {
                    return 0;
                }
                continue;
            }
            for &x in adj[&cur].iter() {
                if x == root {
                    continue;
                }
                if !adj[&root].contains(&x) {
                    return 0;
                }
            }
            if root_degree == cur_degress {
                multiple = true;
            }
        }

        if multiple {
            2
        } else {
            1
        }
    }
}

fn main() {}
