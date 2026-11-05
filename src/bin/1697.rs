#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut edge_list = edge_list;
        let mut queries: Vec<(i32, i32, i32, usize)> = queries
            .iter()
            .enumerate()
            .map(|(i, q)| (q[0], q[1], q[2], i))
            .collect();

        // Sort edges by distance (weight)
        edge_list.sort_unstable_by_key(|e| e[2]);

        // Sort queries by limit
        queries.sort_unstable_by_key(|q| q.2);

        // Initialize DSU
        let mut parent: Vec<usize> = (0..n as usize).collect();

        // Find function for DSU
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        // Union function for DSU
        fn union(parent: &mut Vec<usize>, x: usize, y: usize) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x != root_y {
                parent[root_x] = root_y;
            }
        }

        let mut result = vec![false; queries.len()];
        let mut edge_index = 0;

        // Process each query
        for (p, q, limit, query_index) in queries {
            // Process all edges with distance < limit
            while edge_index < edge_list.len() && edge_list[edge_index][2] < limit {
                let u = edge_list[edge_index][0] as usize;
                let v = edge_list[edge_index][1] as usize;
                union(&mut parent, u, v);
                edge_index += 1;
            }

            // Check if nodes p and q are connected
            result[query_index] = find(&mut parent, p as usize) == find(&mut parent, q as usize);
        }

        result
    }
}
fn main() {}
