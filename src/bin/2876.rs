struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn count_visited_nodes(edges: Vec<i32>) -> Vec<i32> {
        let sz = edges.len();
        let mut is_in_cycle = vec![true; sz];
        let mut in_degree = vec![0; sz];
        for &e in edges.iter() {
            let e = e as usize;
            in_degree[e] += 1;
        }
        let mut q: VecDeque<usize> = VecDeque::new();
        for (i, &v) in in_degree.iter().enumerate() {
            if v == 0 {
                q.push_back(i);
            }
        }
        while let Some(cur) = q.pop_front() {
            is_in_cycle[cur] = false;
            let v = edges[cur] as usize;
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                q.push_back(v);
            }
        }
        let mut ret = vec![0; sz];

        for i in 0..sz {
            if is_in_cycle[i] && ret[i] == 0 {
                let mut cycle_nodes = vec![];
                let mut cur = i;
                while ret[cur] == 0 {
                    ret[cur] = -1;
                    cycle_nodes.push(cur);
                    cur = edges[cur] as usize;
                }

                let cycle_len = cycle_nodes.len() as i32;

                for &node in cycle_nodes.iter() {
                    ret[node] = cycle_len;
                }
            }
        }
        fn dfs(u: usize, edges: &Vec<i32>, ret: &mut Vec<i32>) -> i32 {
            if ret[u] != 0 {
                return ret[u];
            }
            let v = edges[u] as usize;
            ret[u] = 1 + dfs(v, edges, ret);

            ret[u]
        }
        for i in 0..sz {
            if ret[i] == 0 {
                dfs(i, &edges, &mut ret);
            }
        }
        ret
    }
}

fn main() {}
