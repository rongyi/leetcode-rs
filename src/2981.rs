struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut ans = -1;

        for c in b'a'..=b'z' {
            let mut dp = vec![0; n + 1];
            let mut cnt = vec![0; n + 1];
            for i in 0..n {
                if bytes[i] == c {
                    dp[i + 1] = dp[i] + 1;
                } else {
                    dp[i + 1] = 0;
                }
                cnt[dp[i + 1]] += 1;
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
