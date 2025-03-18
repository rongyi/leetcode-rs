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
        // The Union-Find data structure is used to group positions with the same value
        // that are in the same row or column. This is necessary because positions sharing
        // a row or column must have ranks influenced by each other.
        // The code processes values in ascending order and ensures that same value in same row or column
        // have exactly the same rank. This is achieved through these steps:
        //
        // 1. For each value, we group all positions containing that value
        // 2. If two positions with the same value share a row or column, they're grouped together
        //    using the Union-Find data structure
        // 3. All positions in the same group will be assigned the same rank
        // 4. This rank is determined by taking the maximum of the current ranks of rows and columns
        //    in the group, and then adding 1
        //
        // This ensures that:
        // - Equal values in the same row/column get exactly the same rank
        // - Ranks are assigned in ascending order of values
        // - Ranks respect constraints from previous assignments
        //
        // For each value in the matrix:
        // 1. We create a Union-Find with size m+n (m rows + n columns)
        // 2. If two positions (i1,j1) and (i2,j2) have the same value and share a row (i1=i2)
        //    or column (j1=j2), they must be in the same group
        // 3. The rank of each position is determined by the maximum rank in its group
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

            // We need to fetch max_rank first to determine the appropriate rank for
            // each group of positions with the same value. This step is crucial because:
            // 1. Positions in same row/column with same value must have same rank
            // 2. The rank must be higher than any previous rank in that row/column
            // 3. For each group (connected component), we find the max existing rank
            //    across all its rows and columns, then assign (max + 1) to the whole group
            // This ensures that ranks increase monotonically and satisfy all constraints
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
