#![allow(dead_code)]
struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let sz = graph.len();
        if sz == 1 {
            return 0;
        }
        // all mask eq to 1
        let success = (1 << sz) - 1;
        let mut visited = vec![vec![false; 1 << sz]; sz];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        // start from each node
        for i in 0..sz {
            q.push_back((i, 1 << i));
            visited[i][1 << i] = true;
        }

        let mut step = 0;

        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let (node, mask) = q.pop_front().unwrap();
                for &node_next in graph[node].iter() {
                    let node_next = node_next as usize;
                    let mask_next = mask | (1 << node_next);
                    if mask_next == success {
                        return step + 1;
                    }

                    if !visited[node_next][mask_next] {
                        visited[node_next][mask_next] = true;
                        q.push_back((node_next, mask_next));
                    }
                }
            }

            step += 1;
        }

        step
    }
}

fn main() {}
