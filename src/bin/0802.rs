#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = Vec::new();
        let sz = graph.len();
        // 0, not visted yet,
        // 1, processing
        // 2, done and ok status
        let mut color = vec![0; sz];

        for i in 0..sz {
            if Self::dfs(&graph, i, &mut color) {
                ret.push(i as i32);
            }
        }

        ret
    }

    fn dfs(graph: &Vec<Vec<i32>>, i: usize, color: &mut Vec<i32>) -> bool {
        if color[i] != 0 {
            return color[i] == 2;
        }
        color[i] = 1;

        for &j in graph[i].iter() {
            let j = j as usize;
            if color[j] == 2 {
                continue;
            }
            // shit, we meet again, there's loop existed
            if color[j] == 1 {
                return false;
            }

            if !Self::dfs(graph, j, color) {
                return false;
            }
        }

        color[i] = 2;
        true
    }
}

fn main() {}
