struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut i = 0;

        // 1. Find the end of the strictly increasing prefix
        while i + 1 < n && nums[i] < nums[i + 1] {
            i += 1;
        }

        // If the whole array is strictly increasing
        if i == n - 1 {
            return (n as i64 * (n as i64 + 1)) / 2;
        }

        // 2. Find the start of the strictly increasing suffix
        let mut j = n - 1;
        while j > 0 && nums[j - 1] < nums[j] {
            j -= 1;
        }

        // 3. Count valid removals
        // Initial count:
        // - Removing everything from index 0 to j (suffix only remains)
        // - Removing everything from index 0 to j+1, ..., 0 to n (subsets of suffix)
        // This is (n - j + 1)
        let mut ans: i64 = (n - j + 1) as i64;

        // 4. Two pointers to merge prefix and suffix
        // For each element in the valid prefix, find how many suffix elements can follow it
        let mut r = j;
        for l in 0..=i {
            while r < n && nums[l] >= nums[r] {
                r += 1;
            }
            // All subarrays starting at l+1 and ending at r-1 or further are valid removals
            ans += (n - r + 1) as i64;
        }

        ans
    }
}
fn main() {}
