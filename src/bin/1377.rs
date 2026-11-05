#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut edge_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for e in edges.into_iter() {
            edge_map.entry(e[0]).or_default().push(e[1]);
            edge_map.entry(e[1]).or_default().push(e[0]);
        }
        let mut ret = 0.0;
        Self::dfs(1, -1, target, t, 0, 1, &mut ret, &edge_map);

        ret
    }

    // invert accumulation (multiply) in child number
    fn dfs(
        start: i32,
        parent: i32,
        target: i32,
        time_limit: i32,
        acc: i32,
        invert: i64,
        ret: &mut f64,
        edges: &HashMap<i32, Vec<i32>>,
    ) {
        let mut child_size = 0;
        if let Some(lst) = edges.get(&start) {
            for &next_level in lst.iter() {
                // ignore to parent edge
                if next_level != parent {
                    child_size += 1;
                }
            }
        }
        if start == target {
            if acc == time_limit || (acc < time_limit && child_size == 0) {
                *ret = 1.0 / invert as f64;
            } else {
                *ret = 0.0;
            }
            return;
        }
        // keep going
        for &next in edges.get(&start).unwrap_or(&vec![]) {
            if next != parent {
                Self::dfs(
                    next,
                    start,
                    target,
                    time_limit,
                    acc + 1,
                    invert * child_size,
                    ret,
                    edges,
                );
            }
        }
    }
}

fn main() {}
