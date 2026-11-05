struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();
        let s: Vec<char> = s.chars().collect();
        let words: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        // dp2[l][i] len is l at i pos(left side) can be break?
        let mut dp2 = vec![vec![false; n + 1]; n + 1];

        for l in 1..=n {
            for i in 0..l {
                let cur: String = s[i..l].iter().collect();
                if dp[i] && words.contains(&cur) {
                    dp[l] = true;
                    dp2[l][i] = true;
                }
            }
        }

        let mut path: Vec<String> = Vec::new();

        Self::recur(&s, &words, &dp2, &mut ret, &mut path, n);

        ret
    }

    fn recur(
        s: &[char],
        words: &HashSet<String>,
        dp: &Vec<Vec<bool>>,
        ret: &mut Vec<String>,
        path: &mut Vec<String>,
        end: usize,
    ) {
        if end == 0 {
            let mut sentence = path.clone();
            sentence.reverse();
            ret.push(sentence.join(" "));
            return;
        }

        for i in 0..end {
            if dp[end][i] {
                let cur: String = s[i..end].iter().collect();
                path.push(cur);

                Self::recur(s, words, dp, ret, path, i);

                path.pop();
            }
        }
    }
}

fn main() {}
