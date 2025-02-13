#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let sz = slices.len();
        let k = sz / 3;
        // dp[i][j] first i slice pick j
        fn calculate(slices: &[i32], k: usize) -> i32 {
            let m = slices.len();
            let mut dp = vec![vec![0; k + 1]; m + 1];

            for i in 1..=m {
                for j in 1..=k {
                    // pick
                    let pick = if i >= 2 { dp[i - 2][j - 1] } else { 0 } + slices[i - 1];

                    // not pick, just follow the before
                    let not_pick = dp[i - 1][j];
                    dp[i][j] = pick.max(not_pick);
                }
            }

            dp[m][k]
        }

        let case1 = calculate(&slices[..sz - 1], k);
        let case2 = calculate(&slices[1..], k);

        case1.max(case2)
    }
}

fn main() {}
