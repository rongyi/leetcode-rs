struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize + 1];
        for diss in dislikes.iter() {
            graph[diss[0] as usize].push(diss[1]);
            graph[diss[1] as usize].push(diss[0]);
        }
        let mut paint: HashMap<i32, bool> = HashMap::new();

        for i in 1..=n {
            // bool for two colors, let's say black/white
            if !paint.contains_key(&i) {
                if !Self::dfs(&graph, i, &mut paint, true) {
                    return false;
                }
            }
        }

        true
    }
    fn dfs(
        graph: &Vec<Vec<i32>>,
        cur_node: i32,
        paint: &mut HashMap<i32, bool>,
        cur_color: bool,
    ) -> bool {
        if paint.contains_key(&cur_node) {
            return *paint.get(&cur_node).unwrap() == cur_color;
        }

        paint.insert(cur_node, cur_color);

        for &neib in graph[cur_node as usize].iter() {
            if paint.contains_key(&neib) {
                if paint[&neib] == cur_color {
                    return false;
                }
            } else {
                if !Self::dfs(graph, neib, paint, !cur_color) {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {}
