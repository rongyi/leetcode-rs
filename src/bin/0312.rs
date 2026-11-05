struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut bar_with_one = vec![1];
        bar_with_one.extend(nums);
        bar_with_one.push(1);
        // dp[i][j] exclusive i/j, max value
        // dp[i][j] = max(dp[i][k] + dp[k][j] + nums[i] * nums[k] * nums[j])
        let sz = bar_with_one.len();
        let mut dp = vec![vec![0; sz]; sz];

        for len in 2..sz {
            for i in 0..sz - len {
                let j = i + len;
                for k in i + 1..j {
                    let cur =
                        dp[i][k] + dp[k][j] + bar_with_one[i] * bar_with_one[k] * bar_with_one[j];
                    dp[i][j] = dp[i][j].max(cur);
                }
            }
        }

        dp[0][sz - 1]
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for len in 2..4 {
        for i in 0..=v.len() - len {
            println!("{:?}", &v[i..i + len]);
        }
    }
}
