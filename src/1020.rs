struct Solution;

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m as i32 {
            Self::dfs(&mut grid, i, 0);
            Self::dfs(&mut grid, i, n as i32 - 1);
        }
        for j in 0..n as i32 {
            Self::dfs(&mut grid, 0, j);
            Self::dfs(&mut grid, m as i32 - 1, j);
        }

        grid.into_iter().fold(0, |mut acc, lst| {
            let cur_acc = lst.into_iter().fold(0, |mut acc, cur| {
                acc += cur;
                acc
            });
            acc += cur_acc;
            acc
        })

        // let mut ret = 0;
        // for i in 0..m {
        //     for j in 0..n {
        //         if grid[i][j] == 1 {
        //             ret += 1;
        //         }
        //     }
        // }

        // ret
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) {
        if x < 0
            || x >= grid.len() as i32
            || y < 0
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] == 0
        {
            return;
        }
        grid[x as usize][y as usize] = 0;
        Self::dfs(grid, x - 1, y);
        Self::dfs(grid, x + 1, y);
        Self::dfs(grid, x, y - 1);
        Self::dfs(grid, x, y + 1);
    }
}

fn main() {}
