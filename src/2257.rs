struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;
        // initial all 0
        // guard = 10
        // wall = 100
        // fill = 1000
        let mut grid = vec![vec![0; n]; m];
        // fill wall first
        for g in guards.iter() {
            let (i, j) = (g[0] as usize, g[1] as usize);
            grid[i][j] = 10;
        }

        for w in walls.iter() {
            let (i, j) = (w[0] as usize, w[1] as usize);
            grid[i][j] = 100;
        }

        for g in guards.iter() {
            let (x, y) = (g[0] as usize, g[1] as usize);
            for j in y + 1..n {
                if grid[x][j] == 0 {
                    grid[x][j] = 1000;
                } else if grid[x][j] == 100 || grid[x][j] == 10 {
                    break;
                }
            }
            for j in (0..y).rev() {
                if grid[x][j] == 0 {
                    grid[x][j] = 1000;
                } else if grid[x][j] == 100 || grid[x][j] == 10 {
                    break;
                }
            }

            for i in x + 1..m {
                if grid[i][y] == 0 {
                    grid[i][y] = 1000;
                } else if grid[i][y] == 100 || grid[i][y] == 10 {
                    break;
                }
            }
            for i in (0..x).rev() {
                if grid[i][y] == 0 {
                    grid[i][y] = 1000;
                } else if grid[i][y] == 100 || grid[i][y] == 10 {
                    break;
                }
            }
        }

        let mut total = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    total += 1;
                }
            }
        }

        total
    }
}

fn main() {}
