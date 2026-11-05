struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, mut initial: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut min_cnt = graph.len();
        initial.sort_unstable();

        for &drop_node in initial.iter() {
            let cur_cnt = Self::bfs(&graph, &initial, drop_node);
            if cur_cnt < min_cnt {
                min_cnt = cur_cnt;
                ret = drop_node;
            }
        }

        ret
    }
    fn bfs(grap: &Vec<Vec<i32>>, initial: &Vec<i32>, drop_node: i32) -> usize {
        let mut q: VecDeque<i32> = VecDeque::new();
        let mut visited = HashSet::new();
        // don go this node
        visited.insert(drop_node);

        let mut ret = 0;
        // all other node to queue
        for &node in initial.iter() {
            if node != drop_node {
                q.push_back(node);
            }
        }

        while let Some(cur) = q.pop_front() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur);
            ret += 1;

            for i in 0..grap.len() {
                if i as i32 != cur && grap[cur as usize][i] == 1 {
                    q.push_back(i as i32);
                }
            }
        }

        ret
    }
}

fn main() {}
