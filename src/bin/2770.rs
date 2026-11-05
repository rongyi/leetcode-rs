struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let sz = nums.len();
        let mut dp: Vec<i32> = vec![-1; sz];
        // already on index 0
        dp[0] = 0;

        for i in 0..sz {
            // you can not step on this index
            if dp[i] == -1 {
                continue;
            }
            // let mut wayout = false;
            for j in i + 1..sz {
                if (nums[j] - nums[i]).abs() <= target {
                    dp[j] = dp[j].max(1 + dp[i]);
                }
            }
        }

        dp[sz - 1]
    }
}

fn main() {}
