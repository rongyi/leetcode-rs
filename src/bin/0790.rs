struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn num_tilings(n: i32) -> i32 {
            const MOD: i64 = 1_000_000_007;
            let n = n as usize;

            if n <= 2 {
                return n as i32;
            }

            // dp[i] = ways to tile 2×i board completely
            // dp_partial[i] = ways to tile 2×i board with one square sticking out
            let mut dp = vec![0i64; n + 1];
            let mut dp_partial = vec![0i64; n + 1];

            dp[0] = 1; // Empty board
            dp[1] = 1; // One vertical domino
            dp_partial[1] = 0;

            for i in 2..=n {
                // Full tiling:
                // 1. Vertical domino at the end: dp[i-1]
                // 2. Two horizontal dominoes: dp[i-2]
                // 3. One tromino with a partial state: dp_partial[i-1] * 2
                dp[i] = (dp[i - 1] + dp[i - 2] + 2 * dp_partial[i - 1]) % MOD;

                // Partial tiling (one square sticking out on the right):
                // 1. One horizontal domino extends from previous: dp_partial[i-1]
                // 2. A tromino extends from previous partial: dp_partial[i-1]
                dp_partial[i] = (dp[i - 2] + dp_partial[i - 1]) % MOD;
            }

            dp[n] as i32
        }
    }
}

impl Solution {
    // https://leetcode.com/problems/domino-and-tromino-tiling/solutions/1620809/python-java-c-c-dp-image-visualized-explanation-100-faster-o-n/
    pub fn num_tilings(n: i32) -> i32 {
        let m = 1e9 as i64 + 7;
        let mut dp = vec![0i64; n as usize + 1];
        dp[0] = 1;
        dp[1] = 1;
        if n <= 1 {
            return dp[n as usize] as i32;
        }
        dp[2] = 2;
        for i in 3..=n {
            let i = i as usize;
            dp[i] = (dp[i - 1] * 2 + dp[i - 3]) % m;
        }

        dp[n as usize] as i32
    }
}

fn main() {}
