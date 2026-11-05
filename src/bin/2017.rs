struct Solution;

use std::i64;
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        // second robot collect top right or bottom left
        let mut top_sum: i64 = grid[0].iter().map(|c| *c as i64).sum();
        let mut bottom_sum = 0;

        let mut min_collect = i64::MAX;
        for j in 0..grid[0].len() {
            top_sum -= grid[0][j] as i64;
            // first robot collect first row to i and going down, which aslo collect grid[1][j]
            // so bottom left is grid[1][j - 1], so we just make this min calculation before bootom_sum
            // which means shift 1
            min_collect = min_collect.min(top_sum.max(bottom_sum));
            bottom_sum += grid[1][j] as i64;
        }

        min_collect
    }
}

fn main() {}
