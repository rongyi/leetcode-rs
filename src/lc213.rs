struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz <= 1 {
            return nums[0];
        } else if sz == 2 {
            return nums[0].max(nums[1]);
        }

        let mut dp1 = vec![0; sz - 1];
        dp1[0] = nums[0];
        // two houses can rob one
        dp1[1] = nums[0].max(nums[1]);
        for i in 2..sz - 1 {
            dp1[i] = dp1[i - 1].max(dp1[i - 2] + nums[i]);
        }

        let mut dp2 = vec![0; sz - 1];
        dp2[0] = nums[1];
        // two houses can rob one
        dp2[1] = nums[1].max(nums[2]);
        for i in 2..sz - 1 {
            dp2[i] = dp2[i - 1].max(dp2[i - 2] + nums[i + 1]);
        }

        dp1[sz - 2].max(dp2[sz - 2])
    }
}

fn main() {
    let input = vec![0, 0];
    let ret = Solution::rob(input);
}
