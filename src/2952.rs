struct Solution;

impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let mut coins = coins;
        coins.sort_unstable();

        let mut added = 0;
        let mut reachable = 0;
        let mut i = 0;
        let n = coins.len();
        let target = target as i64;

        while reachable < target {
            if i < n && (coins[i] as i64) <= reachable + 1 {
                reachable += coins[i] as i64;
                i += 1;
            } else {
                let new_coin = (reachable + 1) as i32;
                reachable += new_coin as i64;
                added += 1;
            }
        }

        added
    }
}

fn main() {}
