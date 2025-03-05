#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let n = s.len();

        // Find the leftmost and rightmost positions for each character
        let mut left = vec![n; 26];
        let mut right = vec![0; 26];

        for i in 0..n {
            let idx = (s[i] - b'a') as usize;
            left[idx] = left[idx].min(i);
            right[idx] = right[idx].max(i);
        }

        // Find valid intervals
        let mut intervals = Vec::new();

        for c in 0..26 {
            if left[c] == n {
                continue; // Character doesn't exist in s
            }

            let mut l = left[c];
            let mut r = right[c];
            let mut valid = true;

            // Expand interval to include all occurrences of characters in this substring
            let mut i = l;
            while i <= r && valid {
                let char_idx = (s[i] - b'a') as usize;

                // If the character's left index is less than current left, we need to expand
                if left[char_idx] < l {
                    valid = false;
                } else if right[char_idx] > r {
                    // Expand right boundary and start checking from the new characters
                    r = right[char_idx];
                }

                i += 1;
            }

            if valid {
                intervals.push((l, r));
            }
        }

        // Sort intervals by end position for greedy selection
        intervals.sort_by_key(|&(_, r)| r);

        let mut result = Vec::new();
        let mut last_end = 0;

        for (l, r) in intervals {
            if l >= last_end {
                result.push(String::from_utf8(s[l..=r].to_vec()).unwrap());
                last_end = r + 1;
            }
        }

        result
    }
}
fn main() {}
