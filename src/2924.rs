struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        // Step 1: Initialize in-degree array
        let mut in_degree = vec![0; n];

        // Step 2: Fill in-degree based on edges
        // edge[1] is the "weaker" node receiving the directed edge
        for edge in edges {
            let v = edge[1] as usize;
            in_degree[v] += 1;
        }

        let mut champion = -1;
        let mut count = 0;

        // Step 3: Identify nodes with 0 in-degree
        for i in 0..n {
            if in_degree[i] == 0 {
                count += 1;
                champion = i as i32;
            }
        }

        // If more than one node has 0 in-degree, the champion isn't unique
        if count == 1 {
            champion
        } else {
            -1
        }
    }
}

fn main() {}
