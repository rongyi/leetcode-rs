#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = *arr.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;
            let sum = arr.iter().map(|&x| x.min(mid)).sum::<i32>();
            // If right is inclusive (nums.len() - 1):

            // Use while left <= right
            // Update with right = mid - 1
            // If right is exclusive (nums.len()):

            // Use while left < right
            // Update with right = mid
            // For finding leftmost/rightmost element:

            // Leftmost: When equal, do right = mid
            // Rightmost: When equal, do left = mid + 1
            if sum < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let sum1 = arr.iter().map(|&x| x.min(left)).sum::<i32>();
        let sum2 = arr.iter().map(|&x| x.min(left - 1)).sum::<i32>();

        if (sum1 - target).abs() < (sum2 - target).abs() {
            left
        } else {
            left - 1
        }
    }
}

fn main() {}
