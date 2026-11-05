
struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph.entry(from).or_default().push(to);
            graph.entry(to).or_default().push(from);
        }
        // total edges
        let mut ret: i64 = n as i64 * (n - 1) as i64 / 2;
        let mut visited = vec![false; n as usize];

        for i in 0..n as usize {
            if visited[i] {
                continue;
            }
            let mut q = VecDeque::new();
            q.push_back(i);
            visited[i] = true;
            let mut cur_group = 0;
            while let Some(node) = q.pop_front() {
                cur_group += 1;
                if let Some(neibs) = graph.get(&node) {
                    for &neib in neibs.iter() {
                        if !visited[neib] {
                            visited[neib] = true;
                            q.push_back(neib);
                        }
                    }
                }
            }
            ret -= cur_group * (cur_group - 1) / 2;
        }

        ret
    }
}

fn main() {}
