#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        // graph can be painted by two color
        let mut painted: HashMap<i32, bool> = HashMap::new();

        for i in 0..graph.len() {
            let i = i as i32;
            if !painted.contains_key(&i) {
                if !Self::dfs(&graph, i, &mut painted, true) {
                    return false;
                }
            }
        }

        true
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        cur_node: i32,
        painted: &mut HashMap<i32, bool>,
        cur_color: bool,
    ) -> bool {
        if painted.contains_key(&cur_node) {
            return painted[&cur_node] == cur_color;
        }
        painted.insert(cur_node, cur_color);
        for &nei in graph[cur_node as usize].iter() {
            if painted.contains_key(&nei) {
                if painted[&nei] == cur_color {
                    return false;
                }
            } else {
                if !Self::dfs(graph, nei, painted, !cur_color) {
                    return false;
                }
            }
        }

        return true;
    }
}

fn main() {}
