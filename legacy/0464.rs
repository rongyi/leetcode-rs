struct Solution;
use std::collections::HashMap;


impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total <= 1 {
            return true;
        }
        if desired_total <= max_choosable_integer {
            return true;
        }
        let sum = max_choosable_integer * (max_choosable_integer + 1) / 2;
        if desired_total > sum {
            return false;
        }
        if sum == desired_total {
            return max_choosable_integer % 2 == 1;
        }
        let mut dp: HashMap<i32, bool> = HashMap::new();

        Self::dfs(max_choosable_integer, desired_total, 0, &mut dp)
    }

    // 1 <= maxChoosableInteger <= 20
    // i in 1..=max  if i is choosen we set (1 << i) | mask
    fn dfs(
        max_choosable_integer: i32,
        desired_total: i32,
        pos_mask: i32,
        dp: &mut HashMap<i32, bool>,
    ) -> bool {
        // means my opponent reach the total sum first
        if desired_total <= 0 {
            return false;
        }
        if dp.contains_key(&pos_mask) {
            return dp[&pos_mask];
        }
        for i in 0..max_choosable_integer {
            // is pos is not taken
            // the actual value taken is i + 1
            if (pos_mask & (1 << i)) == 0
                && !Self::dfs(
                    max_choosable_integer,
                    desired_total - i - 1,
                    pos_mask | (1 << i),
                    dp,
                )
            {
                dp.insert(pos_mask, true);
                return true;
            }
        }
        dp.insert(pos_mask, false);
        false
    }
}

fn main() {}
