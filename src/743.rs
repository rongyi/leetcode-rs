#![allow(dead_code)]

struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let sz = n as usize + 1;
        let k = k as usize;
        // index 0 is not used!
        let mut graph: Vec<Vec<_>> = vec![vec![]; sz];
        let mut ttl: Vec<i64> = vec![i32::MAX as i64; sz];
        ttl[k] = 0;
        let mut visited = vec![false; sz];
        for t in times.iter() {
            // directed!
            // node1 -> (node2, weight)
            graph[t[0] as usize].push((t[1] as usize, t[2]));
        }
        // small heap, min in top
        let mut q: BinaryHeap<Reverse<(_, usize)>> = BinaryHeap::new();
        q.push(Reverse((0, k)));

        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let Reverse((_, start_node)) = q.pop().unwrap();
                if visited[start_node] {
                    continue;
                }
                for &(end_node, weight) in graph[start_node].iter() {
                    // current weight
                    if ttl[end_node] > ttl[start_node] + weight as i64 {
                        ttl[end_node] = ttl[start_node] + weight as i64;
                        q.push(Reverse((ttl[end_node], end_node)));
                    }
                }

                visited[start_node] = true;
            }
        }

        // index 0 is not used
        let max_time = *ttl.iter().skip(1).max().unwrap();
        if max_time == i32::MAX as i64 {
            return -1;
        }

        return max_time as i32;
    }
}

fn main() {}
