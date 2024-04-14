struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut sz = nums.len();
        let mut dp = vec![1; sz];
        let mut cnt = vec![1; sz];

        let mut max_len = 0;
        let mut ret = 0;

        for k in 0..sz {
            for i in 0..k {
                // can form lis
                if nums[i] < nums[k] {
                    if dp[k] == dp[i] + 1 {
                        cnt[k] += cnt[i];
                    }
                    if dp[k] < dp[i] + 1 {
                        dp[k] = dp[i] + 1;
                        cnt[k] = cnt[i];
                    }
                }
            }
            if max_len == dp[k] {
                ret += cnt[k];
            }
            if max_len < dp[k] {
                max_len = dp[k];
                ret = cnt[k];
            }
        }

        ret
    }
}

fn main() {}
