struct Solution;

mod ai {
    struct Solution;

    struct DSU {
        parent: Vec<usize>,
        size: Vec<usize>,
    }

    impl DSU {
        fn new(n: usize) -> Self {
            DSU {
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
            let ra = self.find(a);
            let rb = self.find(b);
            if ra == rb {
                return;
            }
            if self.size[ra] < self.size[rb] {
                self.parent[ra] = rb;
                self.size[rb] += self.size[ra];
            } else {
                self.parent[rb] = ra;
                self.size[ra] += self.size[rb];
            }
        }

        fn get_size(&mut self, x: usize) -> usize {
            let root = self.find(x);
            self.size[root]
        }
    }

    impl Solution {
        pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
            let m = grid.len();
            let n = grid[0].len();
            let mut grid = grid;

            // Track which hits actually removed a brick
            let mut removed = vec![false; hits.len()];

            // Remove all hit bricks
            for (i, hit) in hits.iter().enumerate() {
                let x = hit[0] as usize;
                let y = hit[1] as usize;
                if grid[x][y] == 1 {
                    grid[x][y] = 0;
                    removed[i] = true;
                }
            }

            let total = m * n;
            let top = total;
            let mut dsu = DSU::new(total + 1);
            let idx = |x: usize, y: usize| x * n + y;
            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            // Connect remaining bricks
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        if i == 0 {
                            dsu.union(idx(i, j), top);
                        }
                        if i > 0 && grid[i - 1][j] == 1 {
                            dsu.union(idx(i, j), idx(i - 1, j));
                        }
                        if j > 0 && grid[i][j - 1] == 1 {
                            dsu.union(idx(i, j), idx(i, j - 1));
                        }
                    }
                }
            }

            let mut result = vec![0; hits.len()];

            // Process in reverse
            for k in (0..hits.len()).rev() {
                // If this hit didn't actually remove a brick, skip it
                if !removed[k] {
                    continue;
                }

                let x = hits[k][0] as usize;
                let y = hits[k][1] as usize;

                let prev_size = dsu.get_size(top);

                // Restore the brick
                grid[x][y] = 1;
                let pos = idx(x, y);

                if x == 0 {
                    dsu.union(pos, top);
                }

                for &(dx, dy) in &dirs {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if grid[nx][ny] == 1 {
                            dsu.union(pos, idx(nx, ny));
                        }
                    }
                }

                let new_size = dsu.get_size(top);
                let fallen = (new_size - prev_size - 1) as i32;
                result[k] = if fallen < 0 { 0 } else { fallen };
            }

            result
        }
    }
}

impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        // hit them all and in reverse restore order
        for hit in hits.iter() {
            let (x, y) = (hit[0], hit[1]);
            if grid[x as usize][y as usize] == 0 {
                // mark as no op
                grid[x as usize][y as usize] = -1;
            } else {
                // real clear
                grid[x as usize][y as usize] = 0;
            }
        }
        for j in 0..n {
            Self::dfs(&mut grid, 0, j as i32);
        }
        let mut ret = vec![0; hits.len()];

        for (idx, hit) in hits.iter().enumerate().rev() {
            let (x, y) = (hit[0], hit[1]);

            if grid[x as usize][y as usize] == -1 {
                continue;
            }
            // restore
            grid[x as usize][y as usize] = 1;
            let mut need_recount = x == 0;
            // check neibor and find value 2
            if !need_recount {
                for d in [[0, 1], [1, 0], [-1, 0], [0, -1]].into_iter() {
                    let (nx, ny) = (x + d[0], y + d[1]);
                    if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                        continue;
                    }
                    if grid[nx as usize][ny as usize] == 2 {
                        need_recount = true;
                        break;
                    }
                }
            }
            if need_recount {
                ret[idx] = Self::dfs(&mut grid, x, y) - 1;
            }
        }

        ret
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
            return 0;
        }
        if grid[x as usize][y as usize] != 1 {
            return 0;
        }
        // 2 means attach from ceil
        grid[x as usize][y as usize] = 2;

        let mut ret = 1;
        for d in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
            let (nx, ny) = (x + d[0], y + d[1]);
            ret += Self::dfs(grid, nx, ny);
        }

        return ret;
    }
}

fn main() {}
