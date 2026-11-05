struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let black: HashSet<usize> = restricted.into_iter().map(|x| x as usize).collect();
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
        }
        let mut visited = vec![false; n];

        Self::dfs(0, usize::MAX, &mut visited, &graph, &black);

        visited.into_iter().filter(|x| *x).count() as _
    }

    fn dfs(
        cur: usize,
        parent: usize,
        visited: &mut Vec<bool>,
        graph: &Vec<Vec<usize>>,
        black: &HashSet<usize>,
    ) {
        visited[cur] = true;

        for &v in graph[cur].iter() {
            if v == parent || black.contains(&v) || visited[v] {
                continue;
            }
            Self::dfs(v, cur, visited, graph, black);
        }
    }
}

fn main() {}
