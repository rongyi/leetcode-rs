struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let sz = n as usize + 1;
        let k = k as usize;
        let mut graph: Vec<Vec<_>> = vec![vec![]; sz];
        let mut ttl: Vec<i64> = vec![i32::MAX as i64; sz];
        ttl[k] = 0;
        let mut visited = vec![false; sz];
        for t in times.iter() {
            // directed!
            graph[t[0] as usize].push((t[1] as usize, t[2]));
        }
        let mut q: BinaryHeap<Reverse<(_, usize)>> = BinaryHeap::new();
        q.push(Reverse((0, k)));

        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let Reverse((_, u)) = q.pop().unwrap();
                if visited[u] {
                    continue;
                }
                visited[u] = true;

                for &(v, weight) in graph[u].iter() {
                    // new path from u -> v is smaller?
                    if ttl[v] > ttl[u] + weight as i64 {
                        ttl[v] = ttl[u] + weight as i64;
                        q.push(Reverse((ttl[v], v)));
                    }
                }
            }
        }

        let max_time = *ttl.iter().skip(1).max().unwrap();
        if max_time == i32::MAX as i64 {
            return -1;
        }
        return max_time as i32;
    }
}

fn main() {}
