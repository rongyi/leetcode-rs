struct Solution;

impl Solution {
    pub fn num_music_playlists(N: i32, L: i32, K: i32) -> i32 {
        // const int mod = pow(10, 9) + 7;
        let m = 1e9 as i64 + 7;

        let mut dp = vec![vec![0i64; L as usize + 1]; N as usize + 1];
        dp[1][1] = 1;
        for i in 2..=N {
            dp[i as usize][i as usize] = (dp[i as usize - 1][i as usize - 1] * i as i64) % m;
        }

        for n in 1..=N {
            for l in (n + 1)..=L {
                let val = (if n - K > 0 { n - K } else { 0 }) as i64;
                dp[n as usize][l as usize] = ((dp[n as usize][l as usize - 1] * val) % m
                    + (dp[n as usize - 1][l as usize - 1] * n as i64) % m)
                    % m;
            }
        }

        dp[N as usize][L as usize] as i32
    }
}

fn main() {}
