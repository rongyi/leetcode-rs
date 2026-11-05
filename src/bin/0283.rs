struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_w_pos = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[non_zero_w_pos] = nums[i];
                non_zero_w_pos += 1;
            }
        }
        for i in non_zero_w_pos..nums.len() {
            nums[i] = 0;
        }
    }
}

fn main() {}
