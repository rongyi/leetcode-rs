#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        // Find the first element that breaks the non-decreasing order from left
        let mut left = 0;
        while left + 1 < n && arr[left] <= arr[left + 1] {
            left += 1;
        }

        // If the array is already sorted, return 0
        if left == n - 1 {
            return 0;
        }

        // Find the first element that breaks the non-decreasing order from right
        let mut right = n - 1;
        while right > 0 && arr[right - 1] <= arr[right] {
            right -= 1;
        }

        // We can remove either [left+1, n-1] or [0, right-1]
        // But we may also be able to remove a smaller segment
        let mut result = std::cmp::min(n - left - 1, right);

        // Try to merge two sorted arrays and find minimum subarray to remove
        let mut i = 0;
        let mut j = right;
        while i <= left && j < n {
            if arr[i] <= arr[j] {
                // Current merge is valid, calculate length of removed segment
                result = std::cmp::min(result, j - i - 1);
                i += 1;
            } else {
                // Need to include more elements from right
                j += 1;
            }
        }

        result as i32
    }
}

fn main() {}
