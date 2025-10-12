struct Solution;

use std::i32;
impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n as usize + 1];

        for r in roads.iter() {
            let (from, to, weight) = (r[0] as usize, r[1] as usize, r[2]);
            graph[from].push((to, weight));
            graph[to].push((from, weight));
        }
        let mut visited = vec![false; n as usize + 1];

        Self::dfs(1, &graph, &mut visited)
    }
    fn dfs(cur: usize, graph: &Vec<Vec<(usize, i32)>>, visited: &mut Vec<bool>) -> i32 {
        let mut ret = i32::MAX;
        visited[cur] = true;
        for &(next, weight) in graph[cur].iter() {
            if !visited[next] {
                ret = ret.min(Self::dfs(next, graph, visited));
            }
            ret = ret.min(weight);
        }
        ret
    }
}

fn main() {}
