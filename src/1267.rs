#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_cnt = vec![0; m];
        let mut col_cnt = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    row_cnt[i] += 1;
                    col_cnt[j] += 1;
                }
            }
        }
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && (row_cnt[i] > 1 || col_cnt[j] > 1) {
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn main() {}
