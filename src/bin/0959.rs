struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<char>> = grid.into_iter().map(|s| s.chars().collect()).collect();
        let sz = grid.len();
        let mut expand = vec![vec![0; sz * 3]; sz * 3];

        for i in 0..sz {
            for j in 0..sz {
                if grid[i][j] == '/' {
                    expand[i * 3][j * 3 + 2] = 1;
                    expand[i * 3 + 1][j * 3 + 1] = 1;
                    expand[i * 3 + 2][j * 3] = 1;
                } else if grid[i][j] == '\\' {
                    expand[i * 3][j * 3] = 1;
                    expand[i * 3 + 1][j * 3 + 1] = 1;
                    expand[i * 3 + 2][j * 3 + 2] = 1;
                }
            }
        }
        let mut ret = 0;
        let sz = expand.len();
        for i in 0..sz {
            for j in 0..sz {
                if expand[i][j] == 0 {
                    Self::dfs(&mut expand, i as i32, j as i32);
                    ret += 1;
                }
            }
        }

        ret
    }
    fn dfs(expand: &mut Vec<Vec<i32>>, i: i32, j: i32) {
        if i < 0
            || i >= expand.len() as i32
            || j < 0
            || j >= expand.len() as i32
            || expand[i as usize][j as usize] == 1
        {
            return;
        }
        expand[i as usize][j as usize] = 1;
        Self::dfs(expand, i + 1, j);
        Self::dfs(expand, i - 1, j);
        Self::dfs(expand, i, j + 1);
        Self::dfs(expand, i, j - 1);
    }
}

fn main() {}
