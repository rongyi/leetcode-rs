#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        // Problem 1551: Minimum Operations to Make Array Equal
        // n is odd: We need to make all elements equal to n
        // n is even: We need to make all elements equal to n

        // Analysis:
        // We have an array arr = [1, 3, 5, ..., 2*n-1]
        // We want to make all elements equal to the mean value
        // The mean value is n when all elements reach equilibrium
        // For each element < n, we need to add up to n
        // For each element > n, we need to reduce down to n
        // Since operations pair up (one +1 and one -1), we only need to count
        // how many increments are needed for elements below n

        // For n = 3, we have [1, 3, 5]
        // Mean is 3, so we need (3-1) = 2 operations

        // For n = 6, we have [1, 3, 5, 7, 9, 11]
        // Mean is 6, so we need (6-1) + (6-3) + (6-5) = 5 + 3 + 1 = 9 operations

        // This is the sum of (n - (2*i+1)) for i = 0 to n/2-1
        // Which simplifies to n²/4 for even n and (n²-1)/4 for odd n

        if n % 2 == 0 {
            n * n / 4
        } else {
            (n * n - 1) / 4
        }
    }
}

fn main() {}
