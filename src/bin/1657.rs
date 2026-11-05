#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // If the lengths are different, the strings can't be close
        if word1.len() != word2.len() {
            return false;
        }

        // Count character frequencies in both words
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];

        // Track which characters exist in each word
        let mut exists1 = [false; 26];
        let mut exists2 = [false; 26];

        // Count character frequencies in word1
        for c in word1.chars() {
            let idx = (c as u8 - b'a') as usize;
            count1[idx] += 1;
            exists1[idx] = true;
        }

        // Count character frequencies in word2
        for c in word2.chars() {
            let idx = (c as u8 - b'a') as usize;
            count2[idx] += 1;
            exists2[idx] = true;
        }

        // Check if the set of characters is the same
        if exists1 != exists2 {
            return false;
        }

        // Sort frequency arrays to check if they match
        count1.sort();
        count2.sort();

        // If the frequency distributions match, the strings are close
        count1 == count2
    }
}

fn main() {}
