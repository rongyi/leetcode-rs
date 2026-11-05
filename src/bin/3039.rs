struct Solution;

impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut counts = [0; 26];
        let mut last_indices = [0; 26];
        let bytes = s.as_bytes();

        // Step 1: Track frequencies and the last index each char appeared
        for (i, &b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            counts[idx] += 1;
            last_indices[idx] = i;
        }

        // Step 2: Find the maximum frequency
        let max_freq = *counts.iter().max().unwrap_or(&0);

        // Step 3: Collect characters that hit max_freq, sorted by their last occurrence
        let mut result_indices: Vec<usize> = Vec::new();
        for i in 0..26 {
            if counts[i] == max_freq {
                result_indices.push(last_indices[i]);
            }
        }

        // Sort by index to maintain original relative order of last occurrences
        result_indices.sort_unstable();

        // Step 4: Map indices back to characters
        result_indices
            .into_iter()
            .map(|i| bytes[i] as char)
            .collect()
    }
}

fn main() {}
