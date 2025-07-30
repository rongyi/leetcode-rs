struct Solution;

impl Solution {
    /// Given a 0-indexed integer array nums and a target element target,
    /// return a list of the target indices of nums after sorting nums in non-decreasing order.
    /// A target index is an index i such that nums[i] == target.
    ///
    /// # Examples
    ///
    /// ```
    /// let nums = vec![1,2,5,2,3];
    /// let target = 2;
    /// assert_eq!(Solution::target_indices(nums, target), vec![1,2]);
    /// ```
    ///
    /// ```
    /// let nums = vec![1,2,5,2,3];
    /// let target = 3;
    /// assert_eq!(Solution::target_indices(nums, target), vec![3]);
    /// ```
    ///
    /// ```
    /// let nums = vec![1,2,5,2,3];
    /// let target = 5;
    /// assert_eq!(Solution::target_indices(nums, target), vec![4]);
    /// ```
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();
        let mut res = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if num == target {
                res.push(i as i32);
            }
        }
        res
    }
}

fn main() {}
