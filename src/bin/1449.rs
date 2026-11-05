#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let mut dp: Vec<String> = vec!["".to_string(); 50001];
        Self::calc(&cost, target, &mut dp)
    }

    fn calc(cost: &Vec<i32>, target: i32, dp: &mut Vec<String>) -> String {
        if target < 0 {
            return "0".to_string();
        }
        if target == 0 {
            return "".to_string();
        }
        if !dp[target as usize].is_empty() {
            return dp[target as usize].clone();
        }

        dp[target as usize] = "0".to_string();
        for i in 1..=9 {
            let cur = Self::calc(cost, target - cost[i - 1], dp);
            // 从小到大排列，就算是长度一样也替换，因为数字在变大
            if cur != "0".to_string() && cur.len() + 1 >= dp[target as usize].len() {
                dp[target as usize] = i.to_string() + &cur;
            }
        }

        dp[target as usize].clone()
    }
}

fn main() {}
