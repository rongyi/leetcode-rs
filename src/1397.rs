#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let evil: Vec<char> = evil.chars().collect();
        let m = evil.len();

        let mut lps = vec![0; m];
        let mut len = 0;

        for i in 1..m {
            while len > 0 && evil[i] != evil[len] {
                len = lps[len - 1];
            }
            if evil[i] == evil[len] {
                len += 1;
                lps[i] = len;
            }
        }

        fn dp(
            idx: usize,
            is_lower_bound: bool,
            is_upper_bound: bool,
            evil_match_len: usize,
            s1: &[char],
            s2: &[char],
            evil: &[char],
            lps: &[usize],
            memo: &mut Vec<Vec<Vec<Vec<Option<i64>>>>>,
        ) -> i64 {
            // contain evil
            if evil_match_len == evil.len() {
                return 0;
            }
            if idx == s1.len() {
                return 1;
            }

            if let Some(val) =
                memo[idx][is_lower_bound as usize][is_upper_bound as usize][evil_match_len]
            {
                return val;
            }
            let lower_char = if is_lower_bound { s1[idx] } else { 'a' };
            let upper_char = if is_upper_bound { s2[idx] } else { 'z' };

            let mut total = 0;
            for c in lower_char..=upper_char {
                let mut new_evil_match_len = evil_match_len;
                while new_evil_match_len > 0 && c != evil[new_evil_match_len] {
                    new_evil_match_len = lps[new_evil_match_len - 1];
                }
                if c == evil[new_evil_match_len] {
                    new_evil_match_len += 1;
                }
                let new_is_lower_bound = is_lower_bound && (c == s1[idx]);
                let new_is_upper_bound = is_upper_bound && (c == s2[idx]);

                total += dp(
                    idx + 1,
                    new_is_lower_bound,
                    new_is_upper_bound,
                    new_evil_match_len,
                    s1,
                    s2,
                    evil,
                    lps,
                    memo,
                );

                total %= MOD;
            }

            memo[idx][is_lower_bound as usize][is_upper_bound as usize][evil_match_len] =
                Some(total);

            total
        }

        let mut memo = vec![vec![vec![vec![None; m]; 2]; 2]; s1.len()];
        dp(0, true, true, 0, &s1, &s2, &evil, &lps, &mut memo) as i32
    }
}

fn main() {}
