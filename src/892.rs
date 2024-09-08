struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    // top and bottom count
                    ret += grid[i][j] * 4 + 2;
                    if i > 0 {
                        ret -= grid[i][j].min(grid[i - 1][j]) * 2;
                    }
                    if j > 0 {
                        ret -= grid[i][j].min(grid[i][j - 1]) * 2;
                    }
                }
            }
        }

        ret
    }
}

fn main() {}
