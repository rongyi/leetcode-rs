struct Solution;

impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();

        // Base case: if only one element, range is 0
        if n == 1 {
            return 0;
        }

        // Initially, if we add k to all elements, range is nums[n-1] - nums[0]
        let mut ans = nums[n - 1] - nums[0];

        // Try all split points: add k to elements [0..i], subtract k from [i+1..n-1]
        for i in 0..n - 1 {
            let high = (nums[i] + k).max(nums[n - 1] - k);
            let low = (nums[0] + k).min(nums[i + 1] - k);
            ans = ans.min(high - low);
        }

        ans
    }
}

fn main() {}
