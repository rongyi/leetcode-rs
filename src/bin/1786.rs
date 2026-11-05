#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

        const MOD: i32 = 1_000_000_007;
        let n = n as usize;

        // Build adjacency list
        let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n + 1];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let weight = edge[2];
            adj[u].push((v, weight));
            adj[v].push((u, weight));
        }

        // Calculate shortest distance from node n to all other nodes using Dijkstra's algorithm
        let mut dist = vec![i32::MAX; n + 1];
        dist[n] = 0;

        let mut pq = BinaryHeap::new();
        // dist, node index
        pq.push(Reverse((0, n)));

        while let Some(Reverse((d, node))) = pq.pop() {
            if d > dist[node] {
                continue;
            }

            for &(neighbor, weight) in &adj[node] {
                if dist[neighbor] > dist[node] + weight {
                    dist[neighbor] = dist[node] + weight;
                    pq.push(Reverse((dist[neighbor], neighbor)));
                }
            }
        }

        // DFS with memoization to count paths
        let mut memo = HashMap::new();

        fn dfs(
            node: usize,
            target: usize,
            adj: &Vec<Vec<(usize, i32)>>,
            dist: &Vec<i32>,
            memo: &mut HashMap<usize, i32>,
        ) -> i32 {
            if node == target {
                return 1;
            }

            if memo.contains_key(&node) {
                return memo[&node];
            }

            let mut count = 0;

            for &(neighbor, _) in &adj[node] {
                if dist[neighbor] < dist[node] {
                    count = (count + dfs(neighbor, target, adj, dist, memo)) % MOD;
                }
            }

            memo.insert(node, count);
            count
        }

        dfs(1, n, &adj, &dist, &mut memo)
    }
}

fn main() {}
