struct Solution;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(sz: usize) -> Self {
        Self {
            parent: (0..sz).collect(),
            size: vec![1; sz],
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
        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }
}

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap() as usize;
        let mut uf = UnionFind::new(max_num + 1);

        for &num in nums.iter() {
            let mut factor = 2;
            let mut n = num as usize;
            while factor * factor <= n {
                if n % factor == 0 {
                    uf.union(num as usize, factor);

                    while n % factor == 0 {
                        n /= factor;
                    }
                }

                factor += 1;
            }
            if n > 1 {
                uf.union(num as usize, n)
            }
        }
        let mut cnt = vec![0; max_num + 1];
        let mut max_size = 0;

        for &num in nums.iter() {
            let root = uf.find(num as usize);
            cnt[root] += 1;
            max_size = max_size.max(cnt[root]);
        }

        max_size
    }
}

fn main() {}
