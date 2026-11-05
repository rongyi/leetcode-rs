struct Solution;

#[derive(Debug, Default)]
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new() -> Self {
        Self {
            parent: (0..26).collect(),
        }
    }

    fn union(&mut self, c1: char, c2: char) {
        let idx1 = (c1 as u8 - 'a' as u8) as usize;
        let idx2 = (c2 as u8 - 'a' as u8) as usize;
        let px = self.find(idx1);
        let py = self.find(idx2);

        // chain together, with smallest char as root
        if px < py {
            // set px as root, py yield as root
            self.parent[py] = px;
        } else {
            self.parent[px] = py;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut uf = UnionFind::new();
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            uf.union(c1, c2);
        }

        let mut ret = String::new();
        for c in base_str.chars() {
            let idx = (c as u8 - 'a' as u8) as usize;
            let root_idx = uf.find(idx);
            let root_char = ('a' as u8 + root_idx as u8) as char;
            ret.push(root_char);
        }

        ret
    }
}

fn main() {}
