#![allow(dead_code)]


struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for c in connections.iter() {
            graph.entry(c[0]).or_default().push(c[1]);
            graph.entry(c[1]).or_default().push(-c[0]);
        }
        let mut visited = vec![false; n as usize];

        Self::dfs(&graph, &mut visited, 0)
    }

    fn dfs(graph: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>, cur: i32) -> i32 {
        let mut change = 0;
        visited[cur as usize] = true;
        if let Some(nexts) = graph.get(&cur) {
            for &to in nexts.iter() {
                if !visited[to.abs() as usize] {
                    change += Self::dfs(graph, visited, to.abs()) + if to > 0 { 1 } else { 0 };
                }
            }
        }

        change
    }
}

fn main() {}
