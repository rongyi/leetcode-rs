
struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let sz = n as usize;
        let mut red = vec![vec![]; sz];
        let mut blue = vec![vec![]; sz];

        for r in red_edges.iter() {
            red[r[0] as usize].push(r[1]);
        }
        for b in blue_edges.iter() {
            blue[b[0] as usize].push(b[1]);
        }
        // dp[i][0] -> end with red
        // dp[i][1] -> end with blue
        let mut dp = vec![[i32::MAX; 2]; sz];
        dp[0] = [0, 0];

        // (node, color, dist)
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        q.push_back((0, 1, 0));
        while let Some((node, color, dist)) = q.pop_front() {
            let next_edges = if color == 0 { &blue } else { &red };
            let next_color = 1 - color;

            for &next_node in &next_edges[node] {
                if dp[next_node as usize][next_color as usize] == i32::MAX {
                    dp[next_node as usize][next_color as usize] = dist + 1;
                    q.push_back((next_node as usize, next_color, dist + 1));
                }
            }
        }

        (0..sz)
            .map(|i| {
                let min_dist = dp[i][0].min(dp[i][1]);
                if min_dist == i32::MAX {
                    -1
                } else {
                    min_dist
                }
            })
            .collect()
    }
}

fn main() {}
