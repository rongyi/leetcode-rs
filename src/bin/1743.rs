#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

        for p in adjacent_pairs.iter() {
            graph.entry(p[0]).or_default().insert(p[1]);
            graph.entry(p[1]).or_default().insert(p[0]);
        }
        let mut start_node = -1;
        for (k, v) in graph.iter() {
            if v.len() == 1 {
                start_node = *k;
                break;
            }
        }
        let mut ret = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = vec![start_node];

        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            ret.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    stack.push(neighbor);
                }
            }
        }

        ret
    }
}

fn main() {}
