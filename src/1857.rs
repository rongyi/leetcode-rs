#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let sz = colors.len();
        let colors: Vec<usize> = colors.bytes().map(|c| (c - b'a') as usize).collect();

        // dp[node][color] end with index: node which color is color
        let mut dp = vec![vec![0; 26]; sz];
        let mut graph = vec![vec![]; sz];
        let mut in_degreee = vec![0; sz];
        let mut q = VecDeque::new();

        for e in edges.iter() {
            let from = e[0] as usize;
            let to = e[1] as usize;
            in_degreee[to] += 1;
            graph[from].push(to);
        }
        for i in 0..sz {
            if in_degreee[i] == 0 {
                q.push_back(i);
                dp[i][colors[i]] = 1;
            }
        }
        let mut visited = 0;
        let mut max_color_value = 0;

        while let Some(cur_node) = q.pop_front() {
            visited += 1;
            for c in 0..26 {
                max_color_value = max_color_value.max(dp[cur_node][c]);
            }

            for &next in &graph[cur_node] {
                for c in 0..26 {
                    dp[next][c] =
                        dp[next][c].max(dp[cur_node][c] + if colors[next] == c { 1 } else { 0 });
                }
                in_degreee[next] -= 1;
                if in_degreee[next] == 0 {
                    q.push_back(next);
                }
            }
        }

        if visited != sz {
            return -1;
        }

        max_color_value
    }
}

fn main() {}
