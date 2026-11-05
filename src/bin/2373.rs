struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = n - 2;
        let mut ret = vec![vec![0; m]; m];

        for i in 0..m {
            for j in 0..m {
                let mut max_val = 0;

                for dx in 0..3 {
                    for dy in 0..3 {
                        max_val = max_val.max(grid[i + dx][j + dy]);
                    }
                }

                ret[i][j] = max_val;
            }
        }

        ret
    }
}

fn main() {}
