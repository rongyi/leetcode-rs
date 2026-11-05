struct Solution;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let s = pressed_keys.as_bytes();
        let sz = s.len();
        let mut dp = vec![0; sz + 1];
        dp[0] = 1;
        let m = 1e9 as i32 + 7;

        for i in 0..sz {
            // single press, just equal to previous number;
            dp[i + 1] = dp[i] % m;
            // notice the chain if
            if i >= 1 && s[i - 1] == s[i] {
                dp[i + 1] = (dp[i + 1] + dp[i + 1 - 2]) % m;

                if i >= 2 && s[i - 2] == s[i] {
                    dp[i + 1] = (dp[i + 1] + dp[i + 1 - 3]) % m;

                    if i >= 3 && (s[i] == b'7' || s[i] == b'9') && s[i] == s[i - 3] {
                        dp[i + 1] = (dp[i + 1] + dp[i + 1 - 4]) % m;
                    }
                }
            }
        }
        dp[sz]
    }
}
fn main() {}
