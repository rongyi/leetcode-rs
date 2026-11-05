#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut count = 0;
        let mut state = 0;
        let mut freq = vec![0; 1024];
        freq[0] = 1;

        // Initialize a counter for wonderful substrings
        // State is a 10-bit integer where each bit represents the parity of occurrences of a character (a to j)
        // freq[state] counts how many times we've seen each state
        // freq[0] = 1 accounts for the empty substring (all characters have even occurrences)

        for c in word.chars() {
            // Flip the bit corresponding to the character
            state ^= 1 << (c as u8 - b'a');

            // Count substrings with all characters appearing an even number of times
            count += freq[state];

            // Count substrings with exactly one character appearing an odd number of times
            for i in 0..10 {
                let altered_state = state ^ (1 << i);
                count += freq[altered_state];
            }

            // Update frequency of current state
            freq[state] += 1;
        }

        count
    }
}

fn main() {}
