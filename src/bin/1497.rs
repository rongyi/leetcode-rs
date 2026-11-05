#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        // Create a HashMap to store the count of remainder when divided by k
        let mut remainder_count = HashMap::new();

        // Count the remainders when divided by k
        for num in arr {
            // Handle negative numbers by adding k and taking modulo again
            let rem = ((num % k) + k) % k;
            *remainder_count.entry(rem).or_insert(0) += 1;
        }

        // Check if the pairs can be formed
        for i in 0..=k / 2 {
            if i == 0 {
                // For remainder 0, we need an even count
                if remainder_count.get(&0).unwrap_or(&0) % 2 != 0 {
                    return false;
                }
            } else if i == k - i {
                // For k/2 (when k is even), we need an even count
                if remainder_count.get(&i).unwrap_or(&0) % 2 != 0 {
                    return false;
                }
            } else {
                // For others, count of i should be equal to count of (k-i)
                let count_i = *remainder_count.get(&i).unwrap_or(&0);
                let count_k_minus_i = *remainder_count.get(&(k - i)).unwrap_or(&0);

                if count_i != count_k_minus_i {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {}
