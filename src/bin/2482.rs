struct Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret: Vec<Vec<i32>> = vec![vec![0; n]; m];

        let mut one_rows: Vec<i32> = vec![0; m];
        for i in 0..m {
            one_rows[i] = grid[i].iter().filter(|&&x| x == 1).count() as i32;
        }
        let mut one_cols: Vec<i32> = vec![0; n];
        for j in 0..n {
            let mut acc = 0;
            for i in 0..m {
                if grid[i][j] == 1 {
                    acc += 1;
                }
            }
            one_cols[j] = acc;
        }

        for i in 0..m {
            for j in 0..n {
                ret[i][j] = 2 * one_rows[i] + 2 * one_cols[j] - m as i32 - n as i32;
            }
        }

        ret
    }
}

fn main() {}
