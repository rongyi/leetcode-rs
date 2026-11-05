#![allow(dead_code)]

struct Solution;

// leetcode 1473
impl Solution {
    // m house
    // n color
    // target neighborhood
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let target = target as usize;
        // dp[i][j][k]   i house, j color, k neighborhood
        let mut dp = vec![vec![vec![i32::MAX; target + 1]; n + 1]; m + 1];
        // check first house
        // not paint yet
        if houses[0] == 0 {
            // we can paint it with any color
            for j in 1..=n {
                dp[1][j][1] = cost[0][j - 1];
            }
        } else {
            // already painted
            dp[1][houses[0] as usize][1] = 0;
        }
        for i in 2..=m {
            for prev_color in 1..=n {
                for prev_neighborhoods in 1..=target {
                    // invalid states
                    if dp[i - 1][prev_color][prev_neighborhoods] == i32::MAX {
                        continue;
                    }
                    if houses[i - 1] == 0 {
                        // not paint
                        for curr_color in 1..=n {
                            let new_neighborhoods = if curr_color == prev_color {
                                prev_neighborhoods
                            } else {
                                prev_neighborhoods + 1
                            };
                            if new_neighborhoods > target {
                                continue;
                            }
                            dp[i][curr_color][new_neighborhoods] =
                                dp[i][curr_color][new_neighborhoods].min(
                                    dp[i - 1][prev_color][prev_neighborhoods]
                                        + cost[i - 1][curr_color - 1],
                                );
                        }
                    } else {
                        let curr_color = houses[i - 1] as usize;
                        let new_neighborhoods = if curr_color == prev_color {
                            prev_neighborhoods
                        } else {
                            prev_neighborhoods + 1
                        };
                        if new_neighborhoods > target {
                            continue;
                        }
                        dp[i][curr_color][new_neighborhoods] = dp[i][curr_color][new_neighborhoods]
                            .min(dp[i - 1][prev_color][prev_neighborhoods]);
                    }
                }
            }
        }

        let mut ret = i32::MAX;
        for c in 1..=n {
            ret = ret.min(dp[m][c][target]);
        }
        if ret == i32::MAX {
            -1
        } else {
            ret
        }
    }
}

fn main() {}
