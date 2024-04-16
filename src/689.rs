#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let sz = nums.len();
        // dp[i] 表示长度为k，结尾index为i的和
        // 问题归结为 dp[x] + dp[y] + dp[z] 最大
        // y <= z - k ==> y + k <= z
        // x <= y - k ==> x + k <= y
        // 凑一起
        // x + k + k <= y + k <= z
        // x + 2k <= y + k <= z
        // 这样的index需求下求最大
        // candidt_idx 缓存x y z坐标
        // cur 缓存三个数组的和
        // 其中：cur[0] == 第一个数组 cur[1] == 第一 + 第二个数组
        // cur[2] == 第一 + 第二 + 第三
        let mut candidate_idx = vec![vec![]; 3];
        let mut cur = vec![0; 3];
        let mut dp = vec![0; sz];

        let mut sum = 0;
        let k = k as usize;
        for i in 0..sz {
            sum += nums[i];
            if i >= k - 1 {
                dp[i] = sum;
                sum -= nums[i - k + 1];

                if i >= 3 * k - 1 {
                    if dp[i - k - k] > cur[0] {
                        cur[0] = dp[i - k - k];
                        candidate_idx[0] = vec![i - k - k];
                    }
                    if dp[i - k] + cur[0] > cur[1] {
                        cur[1] = dp[i - k] + cur[0];
                        candidate_idx[1] = vec![candidate_idx[0][0], i - k];
                    }
                    if dp[i] + cur[1] > cur[2] {
                        cur[2] = cur[1] + dp[i];
                        candidate_idx[2] = vec![candidate_idx[1][0], candidate_idx[1][1], i];
                    }
                }
            }
        }
        vec![
            (candidate_idx[2][0] - k + 1) as i32,
            (candidate_idx[2][1] - k + 1) as i32,
            (candidate_idx[2][2] - k + 1) as i32,
        ]
    }
}

fn main() {}
