#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let modulo = 1_000_000_007;
        let n = words[0].len();
        let m = target.len();

        if n < m {
            return 0;
        }

        // Create a frequency map of characters at each position
        let mut freq = vec![vec![0; 26]; n];
        for word in &words {
            for (j, c) in word.chars().enumerate() {
                freq[j][(c as u8 - b'a') as usize] += 1;
            }
        }

        // DP array to store the number of ways to form the prefix of target
        let mut dp = vec![vec![0; m + 1]; n + 1];

        // Initialize base case: empty target string can be formed in 1 way
        for i in 0..=n {
            dp[i][0] = 1;
        }

        // Fill DP array
        for i in 1..=n {
            for j in 1..=m {
                // Don't use current position
                dp[i][j] = dp[i - 1][j];

                // Use current position if the character matches
                let char_idx = (target.as_bytes()[j - 1] - b'a') as usize;
                let count = freq[i - 1][char_idx];

                if count > 0 {
                    dp[i][j] = (dp[i][j]
                        + (dp[i - 1][j - 1] as i64 * count as i64) % modulo as i64)
                        % modulo as i64;
                }
            }
        }

        dp[n][m] as i32
    }
}

fn main() {}
