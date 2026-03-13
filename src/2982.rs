struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        // counts[char_index][length] = frequency
        let mut counts = vec![vec![0; n + 1]; 26];

        let mut i = 0;
        while i < n {
            let mut j = i;
            // Find the end of the current "special" block
            while j < n && s[j] == s[i] {
                j += 1;
            }

            let char_idx = (s[i] - b'a') as usize;
            let len = j - i;

            // A block of length 'len' contributes to all shorter lengths
            // e.g., "aaaa" (len 4) contains:
            // 1 x len 4, 2 x len 3, 3 x len 2, 4 x len 1
            for l in 1..=len {
                counts[char_idx][l] += (len - l + 1) as i32;
            }

            i = j;
        }

        let mut max_len = -1;
        for char_idx in 0..26 {
            for l in 1..=n {
                if counts[char_idx][l] >= 3 {
                    max_len = max_len.max(l as i32);
                }
            }
        }

        max_len
    }
}

fn main() {}
