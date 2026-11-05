#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut in_degree = vec![0; n as usize];

        // Count the in-degree of each vertex
        for edge in &edges {
            let to = edge[1] as usize;
            in_degree[to] += 1;
        }

        // Vertices with in-degree of 0 need to be included in the result
        let mut result = Vec::new();
        for (i, &degree) in in_degree.iter().enumerate() {
            if degree == 0 {
                result.push(i as i32);
            }
        }

        result
    }
}

fn main() {}
