struct Solution;

mod ai {
    struct Solution;
    use std::collections::{HashMap, VecDeque};

    impl Solution {
        pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
            let n = n as usize;
            // Build adjacency list
            let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
            for edge in dislikes {
                let a = edge[0] as usize;
                let b = edge[1] as usize;
                graph[a].push(b);
                graph[b].push(a);
            }

            // 0 = uncolored, 1 = group A, -1 = group B
            let mut colors: Vec<i32> = vec![0; n + 1];

            // Try to color each connected component
            for i in 1..=n {
                if colors[i] == 0 {
                    // Start BFS from this node
                    colors[i] = 1;
                    let mut queue = VecDeque::new();
                    queue.push_back(i);

                    while let Some(node) = queue.pop_front() {
                        let current_color = colors[node];
                        let next_color = -current_color; // Opposite color

                        for &neighbor in &graph[node] {
                            if colors[neighbor] == 0 {
                                colors[neighbor] = next_color;
                                queue.push_back(neighbor);
                            } else if colors[neighbor] == current_color {
                                // Conflict: neighbor has same color
                                return false;
                            }
                            // If neighbor has opposite color, it's fine
                        }
                    }
                }
            }

            true
        }
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize + 1];
        for diss in dislikes.iter() {
            graph[diss[0] as usize].push(diss[1]);
            graph[diss[1] as usize].push(diss[0]);
        }
        let mut paint: HashMap<i32, bool> = HashMap::new();

        for i in 1..=n {
            // bool for two colors, let's say black/white
            if !paint.contains_key(&i) {
                if !Self::dfs(&graph, i, &mut paint, true) {
                    return false;
                }
            }
        }

        true
    }
    fn dfs(
        graph: &Vec<Vec<i32>>,
        cur_node: i32,
        paint: &mut HashMap<i32, bool>,
        cur_color: bool,
    ) -> bool {
        if paint.contains_key(&cur_node) {
            return *paint.get(&cur_node).unwrap() == cur_color;
        }

        paint.insert(cur_node, cur_color);

        for &neib in graph[cur_node as usize].iter() {
            if paint.contains_key(&neib) {
                if paint[&neib] == cur_color {
                    return false;
                }
            } else {
                if !Self::dfs(graph, neib, paint, !cur_color) {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {}
