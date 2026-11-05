struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        let words: HashSet<String> = word_dict.into_iter().collect();
        dp[0] = true;

        for j in 1..=n {
            for i in 0..j {
                // break from the left side of i
                // the first part can be word break can be expressed as dp[i] dp[i - 1 + 1]
                // the rest if can be found in dict, then this is a valid part
                let rest: String = s[i..j].iter().collect();
                if dp[i] && words.contains(&rest) {
                    dp[j] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}

fn main() {}
