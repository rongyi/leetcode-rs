#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        let n = arr.len();
        let to_remove = n / 20; // 5% from each end

        // Sort the array
        arr.sort_unstable();

        // Trim the array by removing 5% from each end
        let trimmed = &arr[to_remove..n - to_remove];

        // Calculate the mean of the remaining elements
        let sum: i32 = trimmed.iter().sum();
        sum as f64 / trimmed.len() as f64
    }
}
fn main() {}
