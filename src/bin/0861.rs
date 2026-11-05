struct Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        // make each row's highest bit be 1
        for i in 0..m {
            if grid[i][0] == 1 {
                continue;
            }

            // flip current row
            for j in 0..n {
                grid[i][j] = 1 - grid[i][j];
            }
        }

        for j in 1..n {
            // cnt how major be 1 or 0, if 0 is major, toggle this col
            let mut one_cnt = 0;
            for i in 0..m {
                if grid[i][j] == 1 {
                    one_cnt += 1;
                }
            }
            // 0 is major, so toggle
            if one_cnt < (m + 1) / 2 {
                // toggle this col
                for i in 0..m {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
        let mut ret = 0;

        for i in 0..m {
            let mut val = 0;
            for j in 0..n {
                val = (val << 1) + grid[i][j];
            }
            ret += val;
        }
        ret
    }
}
fn main() {}
