#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        // Initialize DSU for both Alice and Bob
        let mut uf_alice = UnionFind::new(n);
        let mut uf_bob = UnionFind::new(n);

        let mut used_edges = 0;
        let total_edges = edges.len() as i32;

        // First process type 3 edges (can be used by both Alice and Bob)
        for edge in &edges {
            if edge[0] == 3 {
                let u = edge[1] as usize - 1;
                let v = edge[2] as usize - 1;

                let alice_union = uf_alice.union(u, v);
                let bob_union = uf_bob.union(u, v);

                // yes, this edge is useful
                if alice_union || bob_union {
                    used_edges += 1;
                }
            }
        }

        // Process type 1 edges (can only be used by Alice)
        for edge in &edges {
            if edge[0] == 1 {
                let u = edge[1] as usize - 1;
                let v = edge[2] as usize - 1;

                if uf_alice.union(u, v) {
                    used_edges += 1;
                }
            }
        }

        // Process type 2 edges (can only be used by Bob)
        for edge in &edges {
            if edge[0] == 2 {
                let u = edge[1] as usize - 1;
                let v = edge[2] as usize - 1;

                if uf_bob.union(u, v) {
                    used_edges += 1;
                }
            }
        }

        // Check if the entire graph is connected for both Alice and Bob
        if uf_alice.count != 1 || uf_bob.count != 1 {
            return -1;
        }

        // Return the number of redundant edges
        total_edges - used_edges
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }

        UnionFind {
            parent,
            rank: vec![0; n],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        self.count -= 1;
        true
    }
}

fn main() {}
