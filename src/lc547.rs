
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let n = is_connected.len();
        let mut visited = vec![false; n];

        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..n {
            for j in i..n {
                if is_connected[i][j] == 1 {
                    graph.entry(i).or_insert(HashSet::new()).insert(j);
                    graph.entry(j).or_insert(HashSet::new()).insert(i);
                }
            }
        }

        for i in 0..n {
            if !visited[i] {
                ret += 1;
                Self::dfs(&graph, &mut visited, i);
            }
        }

        ret
    }

    fn dfs(graph: &HashMap<usize, HashSet<usize>>, visited: &mut Vec<bool>, i: usize) {
        visited[i] = true;
        if let Some(v) = graph.get(&i) {
            for &next in v.iter() {
                if !visited[next] {
                    Self::dfs(graph, visited, next);
                }
            }
        }
    }
}

fn main() {}
