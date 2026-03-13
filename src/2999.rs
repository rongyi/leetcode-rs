struct Solution;

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        // Helper function to count powerful numbers from 0 to x (inclusive)
        fn count_up_to(x: i64, limit: i32, s: &str) -> i64 {
            let x_str = x.to_string();
            let n = x_str.len();
            let m = s.len();
            let s_digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();

            // If x is smaller than the smallest number formed by prefixing digits to s, return 0
            if n < m {
                return 0;
            }

            // DP array: dp[i][tight] = count of valid prefixes of length i
            // tight = 0: prefix is already less than x's prefix (can use any digit 0..limit)
            // tight = 1: prefix equals x's prefix so far (must respect x's digit at position i)
            let mut dp = vec![vec![0; 2]; n + 1];
            dp[0][1] = 1; // Empty prefix, tight=1 (equal so far)

            for i in 0..n {
                let x_digit = (x_str.as_bytes()[i] - b'0') as i32;

                // Transition from tight=0 state
                if dp[i][0] > 0 {
                    // For tight=0, we can use any digit from 0 to limit
                    dp[i + 1][0] += dp[i][0] * (limit + 1) as i64;
                }

                // Transition from tight=1 state
                if dp[i][1] > 0 {
                    // For tight=1, we can use digits from 0 to min(limit, x_digit)
                    for digit in 0..=limit.min(x_digit) {
                        let next_tight = if digit == x_digit { 1 } else { 0 };
                        dp[i + 1][next_tight] += dp[i][1];
                    }
                }
            }

            // Now handle the suffix constraint: last m digits must exactly match s
            let mut result = 0;

            // Case 1: Numbers with length > m
            if n > m {
                // For numbers where the first n-m digits are free (subject to limit constraint),
                // and last m digits are fixed to s

                // Check if s itself is within limit
                if s_digits.iter().all(|&d| d <= limit as u8) {
                    // Count valid prefixes of length n-m
                    result += dp[n - m][0] + dp[n - m][1];

                    // But we must ensure the entire number (prefix + s) <= x
                    // Compare the suffix part with s
                    let x_suffix = &x_str[n - m..];
                    let s_str = s;

                    if x_suffix >= s_str {
                        // If suffix is >= s, all dp[n-m][1] (tight=1) prefixes are valid
                        // (dp[n-m][0] prefixes are already all valid since they're < x's prefix)
                    } else {
                        // If suffix < s, then for tight=1 prefixes, the number would be > x
                        // So we need to subtract those
                        result -= dp[n - m][1];
                    }
                }
            }

            // Case 2: Numbers with length == m (i.e., s itself with possible leading zeros in representation)
            if n == m {
                // Only one number to consider: s itself
                // Check if s <= x and all digits of s <= limit
                if s_digits.iter().all(|&d| d <= limit as u8) {
                    let s_num: i64 = s.parse().unwrap();
                    if s_num <= x {
                        result += 1;
                    }
                }
            }

            result
        }

        // Calculate count from 0 to finish minus count from 0 to start-1
        let finish_count = count_up_to(finish, limit, &s);
        let start_minus_one_count = if start > 0 {
            count_up_to(start - 1, limit, &s)
        } else {
            0
        };

        finish_count - start_minus_one_count
    }
}

fn main() {}
