#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        use std::collections::HashSet;

        // Count the frequency of each character
        let mut freq = vec![0; 26];
        for c in s.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }

        // Sort frequencies in descending order
        freq.sort_by(|a, b| b.cmp(a));

        let mut deletions = 0;
        let mut used_frequencies = HashSet::new();

        for &f in &freq {
            if f == 0 {
                continue;
            }

            let mut current_freq = f;
            // Keep decreasing frequency until we find an unused one or reach 0
            while current_freq > 0 && used_frequencies.contains(&current_freq) {
                current_freq -= 1;
                deletions += 1;
            }

            // Only add to used_frequencies if we found a valid non-zero frequency
            if current_freq > 0 {
                used_frequencies.insert(current_freq);
            }
        }

        deletions
    }
}

fn main() {}
