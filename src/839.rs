struct Solution;
// or union find
struct DisjointSet {
    v: Vec<i32>,
    size: i32,
}

impl DisjointSet {
    fn new(sz: usize) -> Self {
        DisjointSet {
            v: (0..sz).into_iter().map(|i| i as i32).collect::<Vec<i32>>(),
            size: sz as i32,
        }
    }

    fn size(&self) -> i32 {
        self.size
    }

    fn find(&mut self, i: i32) -> i32 {
        if i != self.v[i as usize] {
            self.v[i as usize] = Self::find(self, self.v[i as usize]);
        }

        self.v[i as usize]
    }

    fn join(&mut self, i: i32, j: i32) {
        let ri = Self::find(self, i);
        let rj = Self::find(self, j);
        if ri != rj {
            self.v[ri as usize] = rj;
            self.size -= 1;
        }
    }
}

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let sz = strs.len();
        let mut ds = DisjointSet::new(sz);

        for i in 0..sz {
            for j in i + 1..sz {
                if Self::similar(&strs[i], &strs[j]) {
                    ds.join(i as i32, j as i32);
                }
            }
        }
        ds.size()
    }
    fn similar(a: &str, b: &str) -> bool {
        let mut diff = 0;
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        for i in 0..a.len() {
            if a[i] != b[i] {
                diff += 1;
            }
            if diff > 2 {
                return false;
            }
        }

        true
    }
}

fn main() {}
