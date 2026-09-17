struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut uniq_write_pos = 1;
        for i in 1..sz {
            if nums[i] != nums[uniq_write_pos - 1] {
                nums[uniq_write_pos] = nums[i];
                uniq_write_pos += 1;
            }
        }

        uniq_write_pos as _
    }
}

fn main() {}
