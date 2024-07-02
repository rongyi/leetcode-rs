#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let sz = nums.len();
        let mut prefix = vec![0; sz + 1];
        for (i, &num) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + num;
        }
        if k <= 1 {
            return prefix[sz] as f64 / sz as f64;
        }
        let k = k as usize;
        // dp[i][j]
        // 表示前i个数组元素分j块的值，我们最终要求的就是 dp[sz][k]
        // 怎么递推呢？
        // dp[i][j] = dp[k][j - 1] + (k +1 .. i)的sum / 这个区间的个数
        let mut dp = vec![vec![0.0; k + 1]; sz + 1];

        // only one chunk
        for i in 1..=sz {
            dp[i][1] = prefix[i] as f64 / i as f64;
        }
        let K = k;

        // bottom up
        // 块从2开始，当然能分成k块至少有k个元素，
        // 所以i起始 == k
        // 再拿j出来切割，k - 1块的下限坐标就是至少有k - 1个，所以j终在k - 1
        // 起始在i 左边一位
        for k in 2..=K {
            for i in k..=sz {
                for j in (k - 1..=i - 1).rev() {
                    let val = (prefix[i] - prefix[j]) as f64 / (i - j) as f64;
                    dp[i][k] = dp[i][k].max(dp[j][k - 1] + val);
                }
            }
        }

        dp[sz][k]
    }
}

fn main() {}
