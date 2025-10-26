struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let sz = coins.len();
        let total_edges = edges.len() * 2;
        let mut cut_edge = 0;

        let mut graph = vec![HashSet::new(); sz];
        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph[from].insert(to);
            graph[to].insert(from);
        }
        // first, cut useless node from this tree
        let mut leaf_q = VecDeque::new();
        for i in 0..sz {
            if graph[i].len() == 1 && coins[i] == 0 {
                leaf_q.push_back(i);
            }
        }
        while let Some(cur) = leaf_q.pop_front() {
            if graph[cur].is_empty() {
                continue;
            }
            let p = *graph[cur].iter().next().unwrap();
            graph[cur].remove(&p);
            graph[p].remove(&cur);
            cut_edge += 2;
            if graph[p].len() == 1 && coins[p] == 0 {
                leaf_q.push_back(p);
            }
        }

        // now we drop all useless leaf
        // we now drop two out most layers
        // what we left is all edge we must tranverse
        let mut layers = 2;
        // still use leafq, but now its empty
        // find leaf node again
        for i in 0..sz {
            // delete blindly, don't care have coin or not
            if graph[i].len() == 1 {
                leaf_q.push_back(i);
            }
        }

        while layers > 0 {
            let qsz = leaf_q.len();
            for _ in 0..qsz {
                let cur = leaf_q.pop_front().unwrap();
                if graph[cur].is_empty() {
                    continue;
                }
                // cut again
                let p = *graph[cur].iter().next().unwrap();
                graph[p].remove(&cur);
                graph[cur].remove(&p);
                if graph[p].len() == 1 {
                    leaf_q.push_back(p);
                }
                cut_edge += 2;
            }

            layers -= 1;
        }

        (total_edges - cut_edge) as _
    }
}

fn main() {}
