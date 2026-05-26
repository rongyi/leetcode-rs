struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as _;
        }
        let mut write_pos = 2;

        for j in 2..nums.len() {
            if nums[j] != nums[write_pos - 2] {
                nums[write_pos] = nums[j];
                write_pos += 1;
            }
        }

        write_pos as _
    }
}

fn main() {}
