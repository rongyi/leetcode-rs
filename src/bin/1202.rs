#![allow(dead_code)]

struct Solution;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(sz: usize) -> Self {
        Self {
            parent: (0..sz).collect(), // everyone in its own group, each one's parent, point to itself
        }
    }
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        // chain path within
        self.parent[i] = self.find(self.parent[i]);

        self.parent[i]
    }
    fn mk_pair(&mut self, i: usize, j: usize) {
        let pi = self.find(i);
        let pj = self.find(j);
        if pi != pj {
            // chain together
            self.parent[pj] = pi;
        }
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut uf = UnionFind::new(sz);
        for p in pairs.iter() {
            let (i, j) = (p[0], p[1]);
            let pi = uf.find(i as usize);
            let pj = uf.find(j as usize);
            if pi != pj {
                uf.mk_pair(pi, pj);
            }
        }
        let mut groups = vec![vec![]; sz];
        for i in 0..sz {
            groups[uf.find(i)].push(i);
        }
        for g in groups.iter_mut() {
            if g.len() == 0 {
                continue;
            }
            let mut tmp: Vec<char> = Vec::new();
            for &id in g.iter() {
                tmp.push(s[id]);
            }
            // now tmp is lexical smallest
            // copy to destination
            tmp.sort_unstable();
            // no need to sort, because group id in each group is ordered, because we traverse from start to end
            // g.sort_unstable();
            for i in 0..g.len() {
                s[g[i]] = tmp[i];
            }
        }

        s.into_iter().collect()
    }
}

fn main() {}
