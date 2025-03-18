#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // Create a union-find data structure
        let mut uf = UnionFind::new(n as usize + 1);

        // For each possible divisor greater than threshold
        for d in (threshold + 1)..=n {
            // For each multiple of d, union with d
            let mut multiple = d;
            while multiple <= n {
                uf.union(d as usize, multiple as usize);
                multiple += d;
            }
        }

        // Process queries
        queries
            .iter()
            .map(|query| uf.find(query[0] as usize) == uf.find(query[1] as usize))
            .collect()
    }
}

// Union-Find (Disjoint Set) implementation
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        let rank = vec![0; size];

        // Initialize each element as its own parent
        for i in 0..size {
            parent[i] = i;
        }

        UnionFind { parent, rank }
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

        // Union by rank
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
