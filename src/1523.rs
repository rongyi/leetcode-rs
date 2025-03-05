#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        // Calculate the count of numbers in the range
        let count = high - low + 1;

        // If count is even, then exactly half of the numbers are odd
        if count % 2 == 0 {
            return count / 2;
        }

        // If count is odd, then we need to check if the starting number is odd
        // If low is odd, then (count+1)/2 numbers are odd
        // If low is even, then count/2 numbers are odd
        if low % 2 == 1 {
            return count / 2 + 1;
        } else {
            return count / 2;
        }
    }
}

fn main() {}
