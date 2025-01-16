#![allow(dead_code)]
struct Solution;

struct UnionFind {
    vals: Vec<usize>,
}

impl UnionFind {
    fn new(sz: usize) -> Self {
        Self {
            vals: (0..sz).collect(),
        }
    }
    fn find(&mut self, idx: usize) -> usize {
        // point to it self
        if idx == self.vals[idx] {
            return idx;
        }
        // search and update
        self.vals[idx] = self.find(self.vals[idx]);
        self.vals[idx]
    }
    // a return value to indicate union is a fresh bind
    fn union(&mut self, idx1: usize, idx2: usize) -> bool {
        let p1 = self.find(idx1);
        let p2 = self.find(idx2);
        if p1 != p2 {
            // chain group p2 to p2, merge two groups into one
            self.vals[p2] = p1;
            true
        } else {
            // already in a group
            false
        }
    }
}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let sz = n as usize;
        // edge is not enough
        if connections.len() < sz - 1 {
            return -1;
        }
        let mut uf = UnionFind::new(sz);
        let mut components = sz;
        for c in connections.into_iter() {
            if uf.union(c[0] as usize, c[1] as usize) {
                components -= 1;
            }
        }

        (components - 1) as i32
    }
}

fn main() {}
