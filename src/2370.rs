struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = vec![0; 256];
        let k = k as u8;
        let mut ret = 0;
        for c in s.chars() {
            let idx = c as u8;
            for i in idx - k..=idx + k {
                let i = i as usize;
                dp[idx as usize] = dp[idx as usize].max(dp[i]);
            }
            dp[idx as usize] += 1;
            ret = ret.max(dp[idx as usize]);
        }
        ret
    }
}

fn main() {}
