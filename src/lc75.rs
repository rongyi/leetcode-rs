struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;

        let mut i = 0;

        while i <= r {
            if nums[i as usize] == 0 {
                nums.swap(i as usize, l as usize);
                l += 1;
                i += 1;
            } else if nums[i as usize] == 2 {
                nums.swap(i as usize, r as usize);
                r -= 1;
            } else {
                i += 1;
            }
        }
    }
}
