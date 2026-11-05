struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_max = vec![0; m];
        let mut col_max = vec![0; n];
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                row_max[i] = row_max[i].max(grid[i][j]);
                col_max[j] = col_max[j].max(grid[i][j]);
                if grid[i][j] != 0 {
                    // xy plane projection
                    ret += 1;
                }
            }
        }

        ret += row_max.into_iter().sum::<i32>();
        ret += col_max.into_iter().sum::<i32>();

        ret
    }
}

fn main() {}
