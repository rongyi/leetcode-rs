#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent = vec![0; edges.len() + 1];
        for i in 0..parent.len() {
            parent[i] = i;
        }
        let mut ret = Vec::new();
        for e in edges.iter() {
            let (mut v1, mut v2) = (e[0] as usize, e[1] as usize);

            while parent[v1] != v1 {
                v1 = parent[v1];
            }
            while parent[v2] != v2 {
                v2 = parent[v2];
            }
            if v1 == v2 {
                ret = e.clone();
            }
            // union group
            parent[v1] = v2;
        }

        ret
    }
}

fn main() {}
