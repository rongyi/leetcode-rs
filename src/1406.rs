#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let sz = stone_value.len();
        let mut dp = vec![i32::MIN; sz + 1];
        dp[sz] = 0;

        for i in (0..sz).rev() {
            let mut cur_sum = 0;
            for j in 0..3 {
                if i + j < sz {
                    cur_sum += stone_value[i + j];
                    dp[i] = dp[i].max(cur_sum - dp[i + j + 1]);
                }
            }
        }
        match dp[0] {
            0 => "Tie".to_string(),
            x if x > 0 => "Alice".to_string(),
            _ => "Bob".to_string(),
        }
    }
}

fn main() {}
