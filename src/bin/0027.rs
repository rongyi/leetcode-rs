struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut next_write_pos = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[next_write_pos] = nums[i];
                next_write_pos += 1;
            }
        }

        next_write_pos as i32
    }
}

fn main() {}
