struct Solution;

use std::{collections::VecDeque, i32};
impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let sz = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; sz];
        for e in edges.iter() {
            let (from, to) = (e[0] as usize, e[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
        }
        let mut ret = i32::MAX;
        for i in 0..sz {
            Self::bfs(i, sz, &graph, &mut ret);
        }

        if ret == i32::MAX {
            return -1;
        }

        ret
    }

    fn bfs(cur: usize, sz: usize, graph: &Vec<Vec<usize>>, ret: &mut i32) {
        let mut distance = vec![1e9 as i32; sz];
        distance[cur] = 0;
        let mut parent: Vec<i32> = vec![-1; sz];
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(cur);
        while let Some(cur) = q.pop_front() {
            for &next in graph[cur].iter() {
                if distance[next] == 1e9 as i32 {
                    parent[next] = cur as i32;
                    distance[next] = distance[cur] + 1;

                    q.push_back(next);
                } else if parent[cur] != next as i32 {
                    *ret = (*ret).min(distance[cur] + distance[next] + 1);
                }
            }
        }
    }
}

fn main() {}
