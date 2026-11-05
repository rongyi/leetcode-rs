struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_jump_sofar = 0;
        let mut cur_end = 0;
        for i in 0..nums.len() {
            max_jump_sofar = max_jump_sofar.max(i as i32 + nums[i]);

            if cur_end == i as i32 {
                cur_end = max_jump_sofar;
                if cur_end >= (nums.len() - 1) as i32 {
                    return true;
                }
            }
        }

        false
    }
}
