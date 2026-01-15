struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let m = 1_000_000_007;

        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        let mut i: i32 = 1;
        let k = k as u32;
        loop {
            let cur = i.pow(k) as usize;
            if cur > n {
                break;
            }
            for s in (cur..=n).rev() {
                dp[s] = (dp[s] + dp[s - cur]) % m;
            }

            i += 1;
        }

        dp[n]
    }
}

fn main() {}
