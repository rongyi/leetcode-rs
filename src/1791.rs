#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        // Since the center node is connected to all other nodes,
        // it will appear in every edge in the graph.
        // So, we just need to check the first two edges to find the common node.

        let (a, b) = (edges[0][0], edges[0][1]);
        let (c, d) = (edges[1][0], edges[1][1]);

        if a == c || a == d {
            a
        } else {
            b
        }
    }
}

fn main() {}
