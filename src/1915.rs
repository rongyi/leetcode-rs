#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut count = 0;
        let mut state = 0;
        let mut freq = vec![0; 1024];
        freq[0] = 1;

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
