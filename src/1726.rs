#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        // Create a HashMap to count occurrences of products
        use std::collections::HashMap;

        let mut product_counts = HashMap::new();
        let n = nums.len();

        // Calculate all possible products a*b and store in HashMap
        for i in 0..n {
            for j in i + 1..n {
                let product = nums[i] * nums[j];
                *product_counts.entry(product).or_insert(0) += 1;
            }
        }

        // Calculate the number of valid tuples
        let mut count = 0;
        for &freq in product_counts.values() {
            if freq > 1 {
                // For each pair of number pairs that have the same product,
                // we can form 8 different tuples (2 * C(freq, 2))
                count += 8 * freq * (freq - 1) / 2;
            }
        }

        count
    }
}

fn main() {}
