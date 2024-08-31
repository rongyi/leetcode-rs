struct Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for row in grid.iter_mut() {
            if row[0] == 0 {
                // the first bit contribute most, so we ensure the first bit is always 1
                for j in row.iter_mut() {
                    *j = 1 - *j;
                }
            }
        }

        let mut ret = (1 << (n - 1)) * m as i32;
        for col in 1..n {
            let mut cnt = (0..m).filter(|&row| grid[row][col] == 1).count();
            cnt = cnt.max(m - cnt);
            ret += (1 << (n - 1 - col)) * cnt as i32;
        }

        ret
    }
}

fn main() {}
