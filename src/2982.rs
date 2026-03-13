struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut ans = -1;
        for c in b'a'..=b'z' {
            let mut dp = vec![0; n + 2];
            let mut cnt = vec![0; n + 2];
            for i in (0..n).rev() {
                if s[i] == c {
                    dp[i] = dp[i + 1] + 1;
                } else {
                    dp[i] = 0;
                }
                cnt[dp[i]] += 1;
            }
            for len in (1..=n).rev() {
                cnt[len - 1] += cnt[len];
                if cnt[len] >= 3 {
                    ans = ans.max(len as i32);
                    break;
                }
            }
        }
        ans
    }
}

fn main() {}
