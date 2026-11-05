struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s = s.as_bytes();
        let sz = s.len();
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let mut dp = vec![false; sz + 1];
        // cache[i][j]   len i and lst chunk start is j
        let mut cache = vec![vec![false; sz]; sz + 1];
        dp[0] = true;
        for l in 1..=sz {
            for i in 0..l {
                let cur: String = String::from_utf8_lossy(&s[i..l]).to_string();
                if dp[i] && dict.contains(&cur) {
                    dp[l] = true;
                    cache[l][i] = true;
                }
            }
        }
        let mut cur = vec![];
        let mut out = vec![];
        Self::backtrack(s, sz, &mut cur, &mut out, &cache);

        out
    }
    fn backtrack(
        s: &[u8],
        len: usize,
        cur: &mut Vec<String>,
        out: &mut Vec<String>,
        cache: &Vec<Vec<bool>>,
    ) {
        if len == 0 {
            let mut words = cur.clone();
            words.reverse();
            out.push(words.join(" "));
            return;
        }
        for i in 0..len {
            if cache[len][i] {
                let w = String::from_utf8_lossy(&s[i..len]).to_string();
                cur.push(w);
                Self::backtrack(s, i, cur, out, cache);
                cur.pop();
            }
        }
    }
}

fn main() {}
