#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut removed_one = false;

        for i in 1..n {
            if nums[i - 1] >= nums[i] {
                // We've found a non-increasing pair
                if removed_one {
                    // Already removed one element, can't make it strictly increasing
                    return false;
                }

                removed_one = true;

                // Try removing either nums[i-1] or nums[i]
                // Remove nums[i-1] if it makes the array strictly increasing
                if i == 1 || nums[i - 2] < nums[i] {
                    // Case when removing nums[i-1] works
                    continue;
                }

                // Try removing nums[i]
                if i == n - 1 || nums[i - 1] < nums[i + 1] {
                    // Case when removing nums[i] works
                    continue;
                }

                // Neither removal works
                return false;
            }
        }

        // If we reach here, either the array was already strictly increasing
        // or we were able to make it strictly increasing by removing one element
        true
    }
}

fn main() {}
