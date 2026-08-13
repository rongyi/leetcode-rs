struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        // graph can be painted by two color
        let mut painted: HashMap<i32, bool> = HashMap::new();

        for i in 0..graph.len() {
            let i = i as i32;
            if !painted.contains_key(&i) {
                if !Self::dfs(&graph, i, &mut painted, true) {
                    return false;
                }
            }
        }

        true
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        u: i32,
        painted: &mut HashMap<i32, bool>,
        cur_color: bool,
    ) -> bool {
        if let Some(&c) = painted.get(&u) {
            return c == cur_color;
        }

        painted.insert(u, cur_color);
        for &v in graph[u as usize].iter() {
            if let Some(&vc) = painted.get(&v) {
                if vc == cur_color {
                    return false;
                }
            } else {
                // not visited yet
                if !Self::dfs(graph, v, painted, !cur_color) {
                    return false;
                }
            }
        }

        return true;
    }
}

mod ai {
    struct Solution;
    use std::collections::VecDeque;

    impl Solution {
        pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
            let n = graph.len();
            let mut color = vec![-1; n]; // -1: uncolored, 0: color A, 1: color B

            for start in 0..n {
                if color[start] != -1 {
                    continue; // Already colored
                }

                // Start BFS from this node
                let mut queue = VecDeque::new();
                queue.push_back(start);
                color[start] = 0;

                while let Some(node) = queue.pop_front() {
                    let current_color = color[node];
                    let next_color = 1 - current_color; // Toggle color

                    for &neighbor in &graph[node] {
                        let neighbor = neighbor as usize;
                        if color[neighbor] == -1 {
                            // Color the neighbor with opposite color
                            color[neighbor] = next_color;
                            queue.push_back(neighbor);
                        } else if color[neighbor] != next_color {
                            // Conflict: neighbor has the same color
                            return false;
                        }
                    }
                }
            }

            true
        }
    }
}

fn main() {}
