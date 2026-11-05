#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len() as i32 - 1;

        // This problem is finding the kth missing positive integer.
        // The key idea is calculating how many numbers are missing at each position.
        // For example, if arr[mid] is 5 and mid is 2 (0-indexed), then 5-2-1 = 2 numbers are missing.
        // We use binary search to find the position where the kth missing number would be.
        // If the number of missing values at mid is less than k, we look to the right.
        // Otherwise, we look to the left.
        // At the end, left will be the insertion point for the kth missing value.
        // // For instance, if arr = [2,3,4,7,11] and k = 5:
        // - At arr[0]=2: missing = 2-0-1 = 1 (number 1 is missing)
        // - At arr[1]=3: missing = 3-1-1 = 1 (still just 1 is missing)
        // - At arr[2]=4: missing = 4-2-1 = 1 (still just 1 is missing)
        // - At arr[3]=7: missing = 7-3-1 = 3 (numbers 1,5,6 are missing)
        // - At arr[4]=11: missing = 11-4-1 = 6 (numbers 1,5,6,8,9,10 are missing)
        // Since we need the 5th missing number, which is between arr[3]=7 and arr[4]=11,
        // binary search will eventually set left=4 and right=3.
        // The answer is left + k = 4 + 5 = 9.
        while left <= right {
            let mid = left + (right - left) / 2;
            let missing = arr[mid as usize] - mid - 1;

            if missing < k {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left + k
    }
}

fn main() {}
