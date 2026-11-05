#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = Vec::with_capacity(n);

        for b in 0..n {
            let mut i = 0;
            let mut j = b;
            while i < m {
                // to right
                if grid[i][j] == 1 {
                    if j == n - 1 || grid[i][j + 1] != grid[i][j] {
                        break;
                    }
                    j += 1;
                } else {
                    if j == 0 || grid[i][j - 1] != grid[i][j] {
                        break;
                    }
                    j -= 1;
                }

                i += 1;
            }
            if i == m {
                ret.push(j as i32);
            } else {
                ret.push(-1);
            }
        }

        ret
    }
}
fn main() {}
