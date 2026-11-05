struct Solution;

mod ai {
    struct Solution;
    struct UnionFind {
        parent: Vec<usize>,
        size: Vec<usize>,
    }

    impl UnionFind {
        fn new(n: usize) -> Self {
            UnionFind {
                parent: (0..n).collect(),
                size: vec![1; n],
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]);
            }
            self.parent[x]
        }

        fn union(&mut self, a: usize, b: usize) {
            let root_a = self.find(a);
            let root_b = self.find(b);

            if root_a != root_b {
                if self.size[root_a] < self.size[root_b] {
                    self.parent[root_a] = root_b;
                    self.size[root_b] += self.size[root_a];
                } else {
                    self.parent[root_b] = root_a;
                    self.size[root_a] += self.size[root_b];
                }
            }
        }
    }

    impl Solution {
        pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
            let n = graph.len();
            let mut uf = UnionFind::new(n);

            // Build components
            for i in 0..n {
                for j in i + 1..n {
                    if graph[i][j] == 1 {
                        uf.union(i, j);
                    }
                }
            }

            // Find root for each node
            let mut roots = Vec::with_capacity(n);
            for i in 0..n {
                roots.push(uf.find(i));
            }

            // Count size of each component
            let mut component_size = vec![0; n];
            for &root in &roots {
                component_size[root] += 1;
            }

            // Count infected nodes in each component
            let mut infected_count = vec![0; n];
            for &node in &initial {
                let root = roots[node as usize];
                infected_count[root] += 1;
            }

            // Find best node to remove
            let mut best_node = *initial.iter().min().unwrap();
            let mut max_saved = 0;

            for &node in &initial {
                let root = roots[node as usize];

                if infected_count[root] == 1 {
                    let saved = component_size[root];

                    if saved > max_saved || (saved == max_saved && node < best_node) {
                        max_saved = saved;
                        best_node = node;
                    }
                }
            }

            best_node
        }
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let sz = graph.len();
        let mut parent = vec![-1; sz];
        let mut cnt = HashMap::new();

        // initially each group containt itself, yeah, you get it, its a union group
        for i in 0..sz {
            cnt.insert(i, 1);
        }

        // then union find to join group together
        for i in 0..sz {
            for j in i + 1..sz {
                if graph[i][j] == 1 {
                    let parx = Self::find(&mut parent, i);
                    let pary = Self::find(&mut parent, j);
                    if parx != pary {
                        Self::mkunion(&mut parent, parx, pary);
                        let &numy = cnt.get(&pary).unwrap();
                        // merge ygroup to xgroup
                        cnt.entry(parx).and_modify(|x| {
                            *x += numy;
                        });
                    }
                }
            }
        }

        let mut ret = *initial.iter().min().unwrap();
        let mut seen = HashMap::new();
        for &init in initial.iter() {
            // i.e. its group
            let group = Self::find(&mut parent, init as usize);
            *seen.entry(group).or_insert(0) += 1;
        }
        let mut maxi = 0;

        for &init in initial.iter() {
            let par = Self::find(&mut parent, init as usize);
            // 也就是说init 里面还有个搭子同属于一个 group，所以删除这个节点没有半点好处，因为那个搭子会最终传染到这里，所以要找那个能代表一个 group 的那个独苗
            // 删这种节点对最终感染数有效果
            if seen[&par] == 1 {
                if cnt[&par] > maxi {
                    maxi = cnt[&par];
                    ret = init;
                } else if cnt[&par] == maxi {
                    ret = ret.min(init);
                }
            }
        }

        ret as i32
    }

    fn find(parent: &mut Vec<i32>, x: usize) -> usize {
        if parent[x as usize] == -1 {
            return x;
        }

        Self::find(parent, parent[x] as usize)
    }

    fn mkunion(parent: &mut Vec<i32>, x: usize, y: usize) {
        parent[y] = x as i32;
        parent[x] = -1;
    }
}

fn main() {}
