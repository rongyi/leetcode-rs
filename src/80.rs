struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // at most twice
        // whtat even previous two same or not
        let mut next_write_pos = 2;
        let sz = nums.len();
        if sz <= 2 {
            return sz as _;
        }
        for i in 2..sz {
            if nums[i] != nums[next_write_pos - 2] {
                nums[next_write_pos] = nums[i];
                next_write_pos += 1;
            }
        }
        next_write_pos as _
    }
}

fn main() {}
