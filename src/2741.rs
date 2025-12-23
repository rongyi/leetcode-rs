struct Solution;

impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![-1; 1 << 14]; 14];

        Self::dfs(&nums, 0, 0, &mut dp)
    }

    fn dfs(nums: &Vec<i32>, cur: usize, mask: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if mask == (1 << nums.len()) - 1 {
            return 1;
        }
        if dp[cur][mask as usize] != -1 {
            return dp[cur][mask as usize];
        }
        dp[cur][mask as usize] = 0;

        for (i, &v) in nums.iter().enumerate() {
            // already selected
            if (1 << i) & mask != 0 {
                continue;
            }
            if mask == 0 || (nums[i] % nums[cur] == 0) || (nums[cur] % nums[i] == 0) {
                dp[cur][mask as usize] = (dp[cur][mask as usize]
                    + Self::dfs(nums, i, mask | (1 << i), dp))
                    % 1_000_000_007;
            }
        }

        dp[cur][mask as usize]
    }
}

fn main() {}
