#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut sum = 0;

        for i in 0..n {
            // Calculate how many subarrays include this element
            let left = i + 1; // Number of ways to pick the left boundary
            let right = n - i; // Number of ways to pick the right boundary

            // Calculate the number of odd length subarrays that include this element
            let total = left * right;
            let odd = (total + 1) / 2; // Number of odd-length arrays containing this element

            // Add the contribution of this element to all odd-length subarrays
            sum += arr[i] * odd as i32;
        }

        sum
    }
}

fn main() {}
