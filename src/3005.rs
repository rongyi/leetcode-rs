struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut freq_map = HashMap::new();

        // Count frequency of each element
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        // Find maximum frequency
        let max_freq = freq_map.values().max().unwrap_or(&0);

        // Sum frequencies of elements with maximum frequency
        let mut total = 0;
        for &freq in freq_map.values() {
            if freq == *max_freq {
                total += freq;
            }
        }

        total
    }
}

fn main() {}
