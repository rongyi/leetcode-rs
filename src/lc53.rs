struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ret = nums[0];
        let mut max_sum = nums[0];

        for i in 1..nums.len() {
            max_sum = i32::max(nums[i], max_sum + nums[i]);
            ret = ret.max(max_sum);
        }

        ret
    }
}
