#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;
        let mut i = m - 1;
        let mut j = 0;
        while i < m && j < n {
            if grid[i][j] < 0 {
                ret += n - j;
                i -= 1;
            } else {
                j += 1;
            }
        }

        ret as i32
    }
}
fn main() {}
