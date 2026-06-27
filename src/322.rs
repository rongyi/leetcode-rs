struct Solution;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; amount as usize + 1];
        // 0 need 0 coin
        dp[0] = 0;
        coins.sort_unstable();

        for cur_amount in 1..=amount {
            // try to use c
            for &c in coins.iter() {
                if c <= cur_amount {
                    let prev = (cur_amount - c) as usize;
                    if dp[prev] != i32::MAX {
                        dp[cur_amount as usize] = dp[cur_amount as usize].min(dp[prev] + 1);
                    }
                } else {
                    break;
                }
            }
        }

        if dp[amount as usize] == i32::MAX {
            return -1;
        }

        dp[amount as usize]
    }
}

fn main() {}
