#![allow(dead_code)]

struct Solution;

impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let mut dp = vec![vec![-1; steps as usize + 1]; steps as usize / 2 + 1];

        Self::recur(0, steps, arr_len, &mut dp) as i32
    }

    fn recur(pos: i32, steps: i32, arr_len: i32, dp: &mut Vec<Vec<i64>>) -> i64 {
        // 原点，停在那里
        if pos == 0 && steps == 0 {
            return 1;
        }
        if pos < 0 || pos >= arr_len || steps == 0 || pos > steps {
            return 0;
        }
        if dp[pos as usize][steps as usize] != -1 {
            return dp[pos as usize][steps as usize];
        }
        let r = Self::recur(pos + 1, steps - 1, arr_len, dp) % Self::MOD;
        let l = Self::recur(pos - 1, steps - 1, arr_len, dp) % Self::MOD;
        let s = Self::recur(pos, steps - 1, arr_len, dp) % Self::MOD;
        dp[pos as usize][steps as usize] = (r + l + s) % Self::MOD;

        dp[pos as usize][steps as usize]
    }
}

fn main() {}
