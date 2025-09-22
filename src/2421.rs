struct Solution;

struct UnionFind {
    groups: Vec<usize>,
}

impl UnionFind {
    fn new(sz: usize) -> Self {
        UnionFind {
            groups: (0..sz).collect(),
        }
    }

    fn parent(&mut self, i: usize) -> usize {
        if self.groups[i] == i {
            return i;
        }
        self.groups[i] = self.parent(self.groups[i]);
        self.groups[i]
    }
    fn chain(&mut self, i: usize, j: usize) {
        self.groups[i] = j;
    }
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, mut edges: Vec<Vec<i32>>) -> i32 {
        let mut sz = vals.len();
        let mut uf: UnionFind = UnionFind::new(sz);

        edges.sort_by(|a, b| {
            let vala = vals[a[0] as usize].max(vals[a[1] as usize]);
            let valb = vals[b[0] as usize].max(vals[b[1] as usize]);
            vala.cmp(&valb)
        });
        let mut ret = sz;

        let mut count: Vec<usize> = vec![1; sz];
        for e in edges.iter() {
            let p1 = uf.parent(e[0] as usize);
            let p2 = uf.parent(e[1] as usize);

            if vals[p1] == vals[p2] {
                ret += count[p1] * count[p2];
                count[p1] += count[p2];
                uf.chain(p2, p1);
            } else {
                if vals[p1] > vals[p2] {
                    uf.chain(p2, p1);
                } else {
                    uf.chain(p1, p2);
                }
            }
        }

        ret as _
    }
}

fn main() {}
