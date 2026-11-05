struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_reach = 0;
        let mut jumps = 0;
        let mut curr_end = 0;

        for i in 0..nums.len() - 1 {
            max_reach = max_reach.max(i + nums[i] as usize);
            // within this range to curr_end, just see
            // what we can reach to max and don't update jump step
            // until we reach the curr_end
            if i == curr_end {
                jumps += 1;
                curr_end = max_reach;
                if curr_end >= nums.len() - 1 {
                    break;
                }
            }
        }

        jumps
    }
}
