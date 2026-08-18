struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let full_mask = (1 << n) - 1;

        // Use a 2D array for distance: dist[mask][node]
        let mut dist = vec![vec![i32::MAX; n]; 1 << n];
        let mut queue = VecDeque::new();

        // Initialize: start from each node
        for i in 0..n {
            let mask = 1 << i;
            dist[mask][i] = 0;
            queue.push_back((mask, i));
        }

        while let Some((mask, u)) = queue.pop_front() {
            let d = dist[mask][u];

            // If we've visited all nodes
            if mask == full_mask {
                return d;
            }

            for &v in &graph[u] {
                let v = v as usize;
                let next_mask = mask | (1 << v);

                if dist[next_mask][v] > d + 1 {
                    dist[next_mask][v] = d + 1;
                    queue.push_back((next_mask, v));
                }
            }
        }

        -1
    }
}

fn main() {}
