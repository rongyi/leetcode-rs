struct Solution;

impl SolutionAI {
    pub fn num_decodings(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let bytes = s.as_bytes();
        let n = bytes.len();

        // dp0 = dp[i-2], dp1 = dp[i-1], dp2 = dp[i]
        let mut dp0: i64 = 1;
        let mut dp1: i64 = Self::ways_single(bytes[0]);

        if dp1 == 0 {
            return 0;
        }

        for i in 1..n {
            let single = Self::ways_single(bytes[i]);
            let pair = Self::ways_pair(bytes[i - 1], bytes[i]);

            let dp2 = (dp1 * single + dp0 * pair) % MOD;

            dp0 = dp1;
            dp1 = dp2;
        }

        dp1 as i32
    }

    /// Ways to decode a single character
    fn ways_single(c: u8) -> i64 {
        match c {
            b'0' => 0,
            b'*' => 9,
            _ => 1,
        }
    }

    /// Ways to decode a two-character pair (c1, c2)
    fn ways_pair(c1: u8, c2: u8) -> i64 {
        match (c1, c2) {
            (b'*', b'*') => 15, // 11-19 (9) + 21-26 (6)
            (b'*', c) => {
                if c <= b'6' {
                    2 // 1c or 2c
                } else {
                    1 // only 1c
                }
            }
            (b'1', b'*') => 9, // 11-19
            (b'2', b'*') => 6, // 21-26
            (c, b'*') => 0,
            (c1, c2) => {
                let val = (c1 - b'0') * 10 + (c2 - b'0');
                if (10..=26).contains(&val) {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl Solution {
    // dp[i] = dp[i-1]* f(s.substr(i,1)) + dp[i-2]* f(s.substr(i-1, 2))
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let m: i64 = 1e9 as i64 + 7;
        let s = s.as_bytes();
        let mut f1 = 1i64;
        let mut f2 = Self::count(&s[0..1]);

        for i in 1..n {
            let f3 = f2 * Self::count(&s[i..i + 1]) + f1 * Self::count(&s[i - 1..i + 1]);
            f1 = f2;
            f2 = f3 % m;
        }

        f2 as i32
    }

    fn count(s: &[u8]) -> i64 {
        // single char case
        if s.len() == 1 {
            if s[0] == b'*' {
                return 9;
            }
            if s[0] == b'0' {
                return 0;
            } else {
                return 1;
            }
        }
        // two char case
        // 11 12 ... 19 21 ... 26
        // 20 is not valid, because * can only be 1 ~ 9
        if s == &[b'*', b'*'] {
            return 15;
        } else if s[1] == b'*' {
            // second is  * case
            if s[0] == b'1' {
                return 9;
            } else if s[0] == b'2' {
                return 6;
            }
            // invalid
            return 0;
        } else if s[0] == b'*' {
            // can be 1, 2
            if s[1] <= b'6' {
                return 2;
            }
            // else only 1
            return 1;
        }
        let cur = String::from_utf8(s.to_vec())
            .unwrap()
            .parse::<i32>()
            .unwrap_or(0);
        if cur >= 10 && cur <= 26 {
            return 1;
        }
        return 0;
    }
}

fn main() {}
