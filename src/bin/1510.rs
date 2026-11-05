#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![false; n as usize + 1];
        for i in 1..=n {
            for j in 1..=(i as f64).sqrt() as usize {
                if !dp[i as usize - j * j] {
                    dp[i as usize] = true;
                    break;
                }
            }
        }
        dp[n as usize]
    }
}

fn main() {}
