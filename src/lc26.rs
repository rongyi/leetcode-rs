struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz <= 1 {
            return sz as i32;
        }
        let mut i = 0;
        for j in 1..sz {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }

        (i + 1) as i32
    }
}
