
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let sz = s.len();
        let dict: HashSet<String> = word_dict.into_iter().collect();
        // empty case
        let mut dp = vec![false; sz + 1];
        dp[0] = true;

        for l in 1..=sz {
            // check the last segment
            for i in 0..l {
                let v = s[i..l].to_vec();
                let cur: String = String::from_utf8(v).unwrap();
                if dp[i] && dict.contains(&cur) {
                    dp[l] = true;
                    break;
                }
            }
        }

        dp[sz]
    }
}

fn main() {}
