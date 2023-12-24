
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        // end lenth is i
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        let s = s.chars().collect::<Vec<_>>();
        if s[0] != '0' {
            dp[1] = 1;
        }
        for i in 2..=n {
            let last_one = s[i - 1..i]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let last_two = s[i - 2..i]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if last_one >= 1 {
                dp[i] += dp[i - 1];
            }
            if last_two >= 10 && last_two <= 26 {
                dp[i] += dp[i - 2];
            }
        }

        dp[n]
    }
}

