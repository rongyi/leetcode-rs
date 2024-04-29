#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![vec![-1; 51]; 51]; 51];
        let sz = grid.len();

        0.max(Self::recur(&mut dp, 0, 0, 0, sz, &grid))
    }

    fn recur(
        dp: &mut Vec<Vec<Vec<i32>>>,
        r1: usize,
        c1: usize,
        r2: usize,
        sz: usize,
        grid: &Vec<Vec<i32>>,
    ) -> i32 {
        let c2 = r1 + c1 - r2;
        if r1 >= sz || r2 >= sz || c1 >= sz || c2 >= sz || grid[r1][c1] == -1 || grid[r2][c2] == -1
        {
            return i32::MIN;
        }
        if dp[r1][c1][r2] != -1 {
            return dp[r1][c1][r2];
        }
        if r1 == sz - 1 && c1 == sz - 1 {
            return grid[r1][c1];
        }
        let mut ret = grid[r1][c1];
        if r1 != r2 {
            ret += grid[r2][c2];
        }

        let mut tmp = Self::recur(dp, r1 + 1, c1, r2 + 1, sz, grid).max(Self::recur(
            dp,
            r1,
            c1 + 1,
            r2,
            sz,
            grid,
        ));
        tmp = tmp.max(Self::recur(dp, r1 + 1, c1, r2, sz, grid));
        tmp = tmp.max(Self::recur(dp, r1, c1 + 1, r2 + 1, sz, grid));
        ret += tmp;

        dp[r1][c1][r2] = ret;

        ret
    }
}

fn main() {}
