struct Solution;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let m = 1e9 as i64 + 7;
        let sz = arr.len();
        // dp[i][j] end to index i sum is j
        let mut dp = vec![vec![0; target as usize + 1]; sz];

        for i in 0..sz {
            for j in i + 1..sz {
                let sum = arr[i] + arr[j];
                if sum <= target {
                    dp[j][sum as usize] += 1;
                }
            }
        }
        let mut ret = 0;

        for k in 2..sz {
            if arr[k] <= target {
                for j in 0..k {
                    ret = (ret + dp[j][(target - arr[k]) as usize]) % m;
                }
            }
        }

        ret as i32
    }
}

fn main() {}
