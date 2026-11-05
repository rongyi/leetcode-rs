struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        let sz = n as usize;
        let mut graph = vec![vec![]; sz];
        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
        }
        let mut visited = vec![false; sz];

        for i in 0..sz {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            // find one group and let's check this group
            let mut q = VecDeque::new();
            q.push_back(i);
            let mut edges = 0;
            let mut nodes = 0;
            while let Some(cur) = q.pop_front() {
                nodes += 1;
                // so every edge is been added twice
                edges += graph[cur].len();
                for &next in graph[cur].iter() {
                    if !visited[next] {
                        q.push_back(next);
                        visited[next] = true;
                    }
                }
            }
            // n * (n - 1) / 2 == edge, here edge count doubled
            if nodes * (nodes - 1) == edges {
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
