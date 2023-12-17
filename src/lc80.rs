
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut count = 1;
        let mut i = 1;

        for j in 2..nums.len() {
            if nums[j] != nums[i] || nums[j] != nums[i - 1] {
                i += 1;
                nums[i] = nums[j];
                count += 1
            }
        }

        count
    }
}
