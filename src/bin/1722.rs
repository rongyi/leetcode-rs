#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        use std::collections::{HashMap, HashSet};

        // Create a disjoint set (union-find) data structure
        let n = source.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![0; n];

        // Find function with path compression
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        // Union function with rank optimization
        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);

            if root_x == root_y {
                return;
            }

            if rank[root_x] < rank[root_y] {
                parent[root_x] = root_y;
            } else if rank[root_x] > rank[root_y] {
                parent[root_y] = root_x;
            } else {
                parent[root_y] = root_x;
                rank[root_x] += 1;
            }
        }

        // Build the disjoint set from allowed swaps
        for swap in allowed_swaps {
            let i = swap[0] as usize;
            let j = swap[1] as usize;
            union(&mut parent, &mut rank, i, j);
        }

        // Group indices by their connected component
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            groups.entry(root).or_insert(Vec::new()).push(i);
        }

        // Calculate minimum Hamming distance
        let mut hamming_distance = 0;

        for indices in groups.values() {
            // Count frequencies of values in source and target for this group
            let mut source_freq: HashMap<i32, usize> = HashMap::new();
            let mut target_freq: HashMap<i32, usize> = HashMap::new();

            for &idx in indices {
                *source_freq.entry(source[idx]).or_insert(0) += 1;
                *target_freq.entry(target[idx]).or_insert(0) += 1;
            }

            // Find the common elements between source and target in this group
            let mut common_count = 0;
            for (val, count) in source_freq.iter() {
                let target_count = target_freq.get(val).unwrap_or(&0);
                common_count += count.min(target_count);
            }

            // The mismatches are the size of the group minus common elements
            hamming_distance += indices.len() - common_count;
        }

        hamming_distance as i32
    }
}
fn main() {}
