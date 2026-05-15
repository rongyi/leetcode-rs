struct Solution;

use std::i32;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz <= 1 {
            return 0;
        }
        let mut cur_jump_end = 0;
        let mut far_reach = 0;
        let mut jump = 0;
        for i in 0..sz {
            far_reach = far_reach.max(i + nums[i] as usize);
            if i == cur_jump_end {
                jump += 1;
                cur_jump_end = far_reach;
            }
            // fast path
            if cur_jump_end >= sz - 1 {
                break;
            }
        }

        jump
    }
    pub fn jumpmethod2(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut dp = vec![sz as i32; sz];
        // init po at index, so no jump
        dp[0] = 0;

        for i in 0..sz {
            // try those step
            for j in i + 1..=(i + nums[i] as usize).min(sz - 1) {
                dp[j] = dp[j].min(dp[i] + 1);
            }
        }

        // testcase can all reach to end, no -1 return
        dp[sz - 1]
    }
}

fn main() {}
