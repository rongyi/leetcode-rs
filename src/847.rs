#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let sz = graph.len();
        if sz == 1 {
            return 0;
        }
        // all mask eq to 1
        let success = (1 << sz) - 1;
        let mut visited = vec![vec![false; 1 << sz]; sz];
        let mut frontier = Vec::new();

        for i in 0..sz {
            frontier.push((i, 1 << i));
            visited[i][1 << i] = true;
        }

        let mut step = 0;

        while !frontier.is_empty() {
            let mut new_frontier = Vec::new();
            for &(node, mask) in frontier.iter() {
                for &node_next in graph[node].iter() {
                    let node_next = node_next as usize;
                    let mask_next = mask | (1 << node_next);
                    if mask_next == success {
                        return step + 1;
                    }

                    if !visited[node_next][mask_next] {
                        visited[node_next][mask_next] = true;
                        new_frontier.push((node_next, mask_next));
                    }
                }
            }

            frontier = new_frontier;
            step += 1;
        }

        step
    }
}

fn main() {}
