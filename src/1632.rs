#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::{BinaryHeap, HashMap, HashSet};

        let m = matrix.len();
        let n = matrix[0].len();

        // Step 1: Create a map from value to positions
        let mut value_to_pos: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                value_to_pos
                    .entry(matrix[i][j])
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }

        // Step 2: Sort the values
        let mut unique_values: Vec<i32> = value_to_pos.keys().cloned().collect();
        unique_values.sort();

        // Step 3: Initialize row and column ranks
        let mut row_rank = vec![0; m];
        let mut col_rank = vec![0; n];

        // Step 4: Initialize the result matrix
        let mut result = vec![vec![0; n]; m];

        // Union-Find data structure
        struct UnionFind {
            parent: Vec<usize>,
            rank: Vec<usize>,
        }

        impl UnionFind {
            fn new(size: usize) -> Self {
                let mut parent = Vec::with_capacity(size);
                for i in 0..size {
                    parent.push(i);
                }
                UnionFind {
                    parent,
                    rank: vec![0; size],
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

        // Step 5: Process values in ascending order
        for &val in &unique_values {
            let positions = &value_to_pos[&val];

            // Create a union-find for current positions
            let mut uf = UnionFind::new(m + n);

            // Union rows and columns for same values
            for &(i, j) in positions {
                uf.union(i, j + m);
            }

            // Find the root of each group and calculate the max rank
            let mut group_max_rank: HashMap<usize, i32> = HashMap::new();

            for &(i, j) in positions {
                let root_i = uf.find(i);
                let max_rank = group_max_rank.entry(root_i).or_insert(0);
                *max_rank = (*max_rank).max(row_rank[i]).max(col_rank[j]);
            }

            // Update ranks for all positions in same group
            for &(i, j) in positions {
                let root_i = uf.find(i);
                let rank = group_max_rank[&root_i] + 1;
                result[i][j] = rank;
                row_rank[i] = rank;
                col_rank[j] = rank;
            }
        }

        result
    }
}

fn main() {}
