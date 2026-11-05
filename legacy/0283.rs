struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[non_zero_index] = nums[i];
                non_zero_index += 1;
            }
        }
        for i in non_zero_index..nums.len() {
            nums[i] = 0;
        }
    }
}

fn main() {
    unimplemented!();
}
