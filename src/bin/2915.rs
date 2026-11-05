struct Solution;

impl Solution {
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![-1; target + 1];
        dp[0] = 0;

        for num in nums.into_iter() {
            let cur = num as usize;
            for s in (cur..=target).rev() {
                let parent = s - cur;
                if dp[parent] != -1 {
                    // from parent to here
                    dp[s] = dp[s].max(dp[parent] + 1);
                }
            }
        }

        dp[target]
    }
}

fn main() {}
