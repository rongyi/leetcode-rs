struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut colst_sum = nums[0] + nums[1] + nums[2];
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() - 2 {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let cur_sum = nums[i] + nums[l] + nums[r];
                if (cur_sum - target).abs() < (colst_sum - target).abs() {
                    colst_sum = cur_sum;
                }
                if cur_sum < target {
                    l += 1;
                } else if cur_sum > target {
                    r -= 1;
                } else {
                    return target;
                }
            }
        }

        colst_sum
    }
}
