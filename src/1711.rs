#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counter = HashMap::new();
        let mod_val = 1_000_000_007;
        let mut result = 0;

        for d in deliciousness {
            // Check for powers of 2 from 2^0 to 2^21 (since max possible sum is 2^20 + 2^20 = 2^21)
            for i in 0..=21 {
                let power_of_two = 1 << i;
                let complement = power_of_two - d;

                // If we've seen the complement before, add those counts to our result
                if let Some(&count) = counter.get(&complement) {
                    result = (result + count) % mod_val;
                }
            }

            // Update the count for the current deliciousness value
            *counter.entry(d).or_insert(0) += 1;
        }

        result
    }
}
fn main() {}
