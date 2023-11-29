struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i: i32 = nums.len() as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = nums.len() as i32 - 1;
            while j >= 0 && nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }

            nums.swap(i as usize, j as usize);
        }

        let mut l: usize = i as usize + 1;
        let mut r = nums.len() - 1;

        while l < r {
            nums.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}
