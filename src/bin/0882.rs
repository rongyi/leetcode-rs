
struct Solution;

use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let mut cache: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for ed in edges.iter() {
            cache.entry(ed[0]).or_default().insert(ed[1], ed[2]);
            cache.entry(ed[1]).or_default().insert(ed[0], ed[2]);
        }
        // maxmove, start
        let mut pq: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        pq.push((max_moves, 0));
        let mut visited: HashMap<i32, i32> = HashMap::new();

        while !pq.is_empty() {
            let (left_moves, cur_node) = pq.pop().unwrap();
            if !visited.contains_key(&cur_node) {
                visited.insert(cur_node, left_moves);

                if let Some(neibs) = cache.get(&cur_node) {
                    for (&neib, &subdivide) in neibs.iter() {
                        let check_move = left_moves - subdivide - 1;
                        if check_move >= 0 && !visited.contains_key(&neib) {
                            pq.push((check_move, neib));
                        }
                    }
                }
            }
        }
        let mut ret = visited.len() as i32;
        for ed in edges.iter() {
            let a = if let Some(&v) = visited.get(&ed[0]) {
                v
            } else {
                0
            };
            let b = if let Some(&v) = visited.get(&ed[1]) {
                v
            } else {
                0
            };
            ret += (a + b).min(ed[2]);
        }

        ret
    }
}

fn main() {}
