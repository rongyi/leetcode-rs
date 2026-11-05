struct Solution;

impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        // dp[k][d]  k step to reach number d
        // k - 1 step to reach d - 1 or k - 1 step to reach d + 1
        let mut dp = vec![vec![0; 1001]; 1001];

        fn dfs(k: i32, d: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
            if d >= k {
                if d == k {
                    return 1;
                } else {
                    return 0;
                }
            }
            if dp[k as usize][d as usize] == 0 {
                dp[k as usize][d as usize] =
                    (1 + dfs(k - 1, d + 1, dp) + dfs(k - 1, (d - 1).abs(), dp)) % 1_000_000_007;
            }

            dp[k as usize][d as usize] - 1
        }

        dfs(k, (start_pos - end_pos).abs(), &mut dp)
    }
}

fn main() {}
