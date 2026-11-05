struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
            let n = n as usize;
            let k = k as usize;
            let max_pts = max_pts as usize;

            // If k == 0, Alice already stops at 0 points, probability is 1
            // when alice get k, she stops
            // so before last round, she can get max score: k - 1
            // and she get luck , the last round is max_pts, so this
            // is the right boundry she can get: k - 1 + max_pts
            // absolute maximum possible score Alice can ever reach
            // n is just a given number
            if k == 0 || n >= k + max_pts - 1 {
                return 1.0;
            }

            // dp[i] = probability of reaching exactly i points
            let mut dp = vec![0.0; n + 1];
            dp[0] = 1.0;

            // sliding window sum for efficiency
            let mut window_sum = 1.0; // dp[0] + dp[1] + ... + dp[i-1]

            // dp[i] = probability of reaching exactly i points

            // For i > 0:
            // dp[i] = (dp[i-1] + dp[i-2] + ... + dp[i-max_pts]) / max_pts
            //         BUT only for states j < k (where game hasn't stopped)

            // So for dp[5] with max_pts = 3, we need:
            // dp[5] = (dp[4] + dp[3] + dp[2]) / 3
            for i in 1..=n {
                // dp[i] = (dp[i-1] + dp[i-2] + ... + dp[i-max_pts]) / max_pts
                // where we only consider terms with indices < k (since game stops at k)

                // Add new term to window if i-1 < k
                if i - 1 < k {
                    window_sum += dp[i - 1];
                }

                // Remove old term if i-1 >= max_pts
                if i - 1 >= max_pts {
                    window_sum -= dp[i - 1 - max_pts];
                }

                // The window now contains sum of valid dp[j] for j in [i-max_pts, i-1]
                // where j < k (because we only added indices < k)
                dp[i] = window_sum / max_pts as f64;
            }

            // Sum probabilities of reaching scores between k and n
            let mut result = 0.0;
            for i in k..=n {
                result += dp[i];
            }

            result
        }
    }
}

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        // 游戏停止的区间是[k, k - 1 + max_pts]
        // 拿k-1游戏继续然后取了个最大值
        if k == 0 || n >= k + max_pts {
            return 1.0;
        }
        let mut dp = vec![0.0; n as usize + 1];
        dp[0] = 1.0;

        let mut wsum = 1.0;
        let mut ret = 0.0;
        for i in 1..=(n as usize) {
            dp[i] = wsum / max_pts as f64;
            if i < k as usize {
                wsum += dp[i];
            } else {
                ret += dp[i];
            }
            if (i as i32 - max_pts) >= 0 {
                wsum -= dp[i - max_pts as usize];
            }
        }

        ret
    }
}

fn main() {}
