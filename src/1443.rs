#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for e in edges.iter() {
            graph.entry(e[0]).or_default().push(e[1]);
            graph.entry(e[1]).or_default().push(e[0]);
        }
        Self::dfs(&graph, &has_apple, 0, 0, -1) * 2
    }

    fn dfs(
        graph: &HashMap<i32, Vec<i32>>,
        hash_apple: &[bool],
        cur_node: i32,
        cur_height: i32,
        parent: i32,
    ) -> i32 {
        let mut ret = 0;
        if let Some(nexts) = graph.get(&cur_node) {
            for &c in nexts.iter() {
                if c != parent {
                    let child_sum = Self::dfs(graph, hash_apple, c, cur_height + 1, cur_node);
                    if child_sum > 0 {
                        ret += child_sum - cur_height;
                    }
                }
            }
        }
        if ret > 0 || hash_apple[cur_node as usize] {
            return ret + cur_height;
        }

        0
    }
}

fn main() {}
