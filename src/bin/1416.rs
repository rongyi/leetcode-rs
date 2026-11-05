#![allow(dead_code)]

struct Solution;

impl Solution {
    // 全篇背诵！
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut dp = vec![0; sz + 1];
        dp[0] = 1;

        for i in 1..=sz {
            let mut num = 0;
            let mut base = 1;
            for j in (0..i).rev().take(10) {
                let digit = s[j] as i64 - '0' as i64;
                num += digit * base;
                base *= 10;

                if num > k as i64 {
                    break;
                }
                if s[j] == '0' {
                    continue;
                }
                if num >= 1 && num <= k as i64 {
                    dp[i] = (dp[i] + dp[j]) % (1e9 as i64 + 7)
                }
            }
        }

        dp[sz] as i32
    }
}

fn main() {}
