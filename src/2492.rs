struct Solution;

use std::{collections::VecDeque, i32};
impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n as usize + 1];

        for r in roads.iter() {
            let (from, to, weight) = (r[0] as usize, r[1] as usize, r[2]);
            graph[from].push((to, weight));
            graph[to].push((from, weight));
        }
        let mut visited = vec![false; n as usize + 1];

        let mut q: VecDeque<(usize, i32)> = VecDeque::new();
        q.push_back((1, i32::MAX));
        let mut ret = i32::MAX;
        while let Some((cur, weight)) = q.pop_front() {
            visited[cur] = true;
            ret = ret.min(weight);
            for &(next, next_weight) in graph[cur].iter() {
                if visited[next] {
                    continue;
                }
                q.push_back((next, ret.min(next_weight)));
            }
        }

        ret
    }
    fn dfs(cur: usize, graph: &Vec<Vec<(usize, i32)>>, visited: &mut Vec<bool>) -> i32 {
        let mut ret = i32::MAX;
        visited[cur] = true;
        for &(next, weight) in graph[cur].iter() {
            ret = ret.min(weight);
            if visited[next] {
                continue;
            }
            ret = ret.min(Self::dfs(next, graph, visited));
        }
        ret
    }
}

fn main() {}
