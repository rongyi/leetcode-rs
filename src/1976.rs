struct Solution;

use std::{collections::BinaryHeap, i64};
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for r in roads.iter() {
            graph[r[0] as usize].push((r[1] as usize, r[2] as i64));
            graph[r[1] as usize].push((r[0] as usize, r[2] as i64));
        }
        // dijkstra

        Self::dijkstra(&graph, n as usize, 0) as _
    }
    fn dijkstra(graph: &Vec<Vec<(usize, i64)>>, sz: usize, start: usize) -> i64 {
        let MOD: i64 = 1_000_000_000 + 7;
        let mut min_vec: Vec<i64> = vec![i64::MAX; sz];
        let mut ways: Vec<i64> = vec![0; sz];
        ways[start] = 1; // one way
        min_vec[start] = 0; // start cost nothing

        // (cost, node)
        let mut q: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        q.push((0, 0));
        while let Some((cur_cost, cur_node)) = q.pop() {
            let cur_cost = -cur_cost;

            if cur_cost > min_vec[cur_node] {
                continue;
            }
            for &(next_node, path_cost) in graph[cur_node].iter() {
                if min_vec[next_node] > cur_cost + path_cost {
                    min_vec[next_node] = cur_cost + path_cost;
                    ways[next_node] = ways[cur_node];
                    q.push((-min_vec[next_node], next_node));
                } else if min_vec[next_node] == cur_cost + path_cost {
                    ways[next_node] = (ways[next_node] + ways[cur_node]) % MOD;
                }
            }
        }

        ways[sz - 1]
    }
}

fn main() {}
