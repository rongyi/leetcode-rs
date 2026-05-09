struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut uniq_len = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[uniq_len - 1] {
                nums[uniq_len] = nums[i];
                uniq_len += 1;
            }
        }
        uniq_len as _
    }
}

fn main() {}
