#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Store original indices
        let mut edges_with_indices: Vec<(usize, Vec<i32>)> =
            edges.into_iter().enumerate().collect();

        // Sort by weight
        edges_with_indices.sort_by(|a, b| a.1[2].cmp(&b.1[2]));

        // Find the MST weight using Kruskal's algorithm
        let mst_weight = Self::find_mst_weight(n as usize, &edges_with_indices, None, None);

        let mut critical_edges = Vec::new();
        let mut pseudo_critical_edges = Vec::new();

        // Check each edge
        for (idx, (original_idx, edge)) in edges_with_indices.iter().enumerate() {
            // Try to build MST without this edge
            let weight_without_edge =
                Self::find_mst_weight(n as usize, &edges_with_indices, Some(idx), None);

            // If MST weight increases or MST can't be formed, it's a critical edge
            if weight_without_edge > mst_weight || weight_without_edge == -1 {
                critical_edges.push(*original_idx as i32);
            } else {
                // Try to build MST with this edge forced to be included
                let weight_with_edge =
                    Self::find_mst_weight(n as usize, &edges_with_indices, None, Some(idx));

                // If MST weight remains the same, it's a pseudo-critical edge
                if weight_with_edge == mst_weight {
                    pseudo_critical_edges.push(*original_idx as i32);
                }
            }
        }

        vec![critical_edges, pseudo_critical_edges]
    }

    // Kruskal's algorithm with options to exclude/include specific edges
    fn find_mst_weight(
        n: usize,
        edges: &Vec<(usize, Vec<i32>)>,
        exclude_idx: Option<usize>,
        include_idx: Option<usize>,
    ) -> i32 {
        let mut dsu = DisjointSet::new(n);
        let mut weight = 0;
        let mut edge_count = 0;

        // If we need to include an edge, add it first
        if let Some(include_idx) = include_idx {
            let (_, edge) = &edges[include_idx];
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];

            dsu.union(u, v);
            weight += w;
            edge_count += 1;
        }

        // Process all edges in sorted order
        for (idx, (_, edge)) in edges.iter().enumerate() {
            // Skip the excluded edge
            if Some(idx) == exclude_idx {
                continue;
            }

            // Skip if this is the edge we've already included
            if Some(idx) == include_idx {
                continue;
            }

            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];

            // If this edge doesn't create a cycle, add it to MST
            if dsu.find(u) != dsu.find(v) {
                dsu.union(u, v);
                weight += w;
                edge_count += 1;
            }
        }

        // If we couldn't form a spanning tree, return -1
        if edge_count != n - 1 {
            return -1;
        }

        weight
    }
}

// Disjoint Set Union (DSU) implementation
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        DisjointSet {
            parent,
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
    }
}

fn main() {}
