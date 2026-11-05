struct Solution;

pub struct UnionFind {
    pub vals: Vec<usize>,
}

impl UnionFind {
    pub fn new(sz: usize) -> Self {
        let init: Vec<usize> = (0..sz).into_iter().collect();
        Self { vals: init }
    }

    pub fn bind(&mut self, u: usize, v: usize) {
        let pu = self.parent(u);
        let pv = self.parent(v);

        self.vals[pu] = pv;
    }

    pub fn parent(&mut self, u: usize) -> usize {
        // parent is it self
        if self.vals[u] == u {
            return u;
        }
        self.vals[u] = self.parent(self.vals[u]);

        self.vals[u]
    }
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut uf = UnionFind::new(n as usize);
        for e in edges.iter() {
            uf.bind(e[0] as usize, e[1] as usize);
        }

        uf.parent(source as usize) == uf.parent(destination as usize)
    }
}

fn main() {}
