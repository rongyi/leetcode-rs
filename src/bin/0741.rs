struct Solution;

mod ai {
    struct Solution;

    use std::cmp::max;

    impl Solution {
        pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
            let n = grid.len();
            if n == 0 || grid[0][0] == -1 {
                return 0;
            }

            // just two person start from 0,0,  second path reverse is the return journey
            // dp[r1][c1][c2] = max cherries for two paths ending at (r1,c1) and (r2,c2)
            // where r2 = r1 + c1 - c2
            let mut dp = vec![vec![vec![-1; n]; n]; n];
            dp[0][0][0] = grid[0][0];

            for r1 in 0..n {
                for c1 in 0..n {
                    for c2 in 0..n {
                        if dp[r1][c1][c2] < 0 {
                            continue;
                        }

                        let r2 = r1 + c1 - c2;
                        if r2 >= n {
                            continue;
                        }

                        let current = dp[r1][c1][c2];

                        // Try all 4 possible combinations of moves
                        for d1 in &[(0, 1), (1, 0)] {
                            // right, down for person 1
                            for d2 in &[(0, 1), (1, 0)] {
                                // right, down for person 2
                                let nr1 = r1 + d1.0;
                                let nc1 = c1 + d1.1;
                                let nr2 = r2 + d2.0;
                                let nc2 = c2 + d2.1;

                                if nr1 < n
                                    && nc1 < n
                                    && nr2 < n
                                    && nc2 < n
                                    && grid[nr1][nc1] != -1
                                    && grid[nr2][nc2] != -1
                                {
                                    let mut add = grid[nr1][nc1];
                                    if nr1 != nr2 || nc1 != nc2 {
                                        add += grid[nr2][nc2];
                                    }

                                    dp[nr1][nc1][nc2] = max(dp[nr1][nc1][nc2], current + add);
                                }
                            }
                        }
                    }
                }
            }

            max(0, dp[n - 1][n - 1][n - 1])
        }
    }
}

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
