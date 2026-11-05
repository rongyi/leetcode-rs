struct Solution;

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut prev_row: Vec<i32> = grid[0].clone();
        let mut next_row: Vec<i32> = vec![i32::MAX; n];

        for i in 1..m {
            for j in 0..n {
                let prev_val = grid[i - 1][j] as usize;
                let prev_weight = prev_row[j];

                // follow the route
                for (cur_col, &cost) in move_cost[prev_val].iter().enumerate() {
                    // come to current row with index cur_col
                    let val = grid[i][cur_col] + cost + prev_weight;
                    next_row[cur_col] = next_row[cur_col].min(val);
                }
            }
            prev_row = next_row;
            next_row = vec![i32::MAX; n];
        }
        prev_row.into_iter().min().unwrap()
    }
}

fn main() {}
