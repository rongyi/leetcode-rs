#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        // Initialize counts for each vowel
        let mut a = 1; // Strings ending with 'a'
        let mut e = 1; // Strings ending with 'e'
        let mut i = 1; // Strings ending with 'i'
        let mut o = 1; // Strings ending with 'o'
        let mut u = 1; // Strings ending with 'u'

        // For each length from 2 to n
        for _ in 1..n {
            // Update counts based on the previous counts
            // For each new character, we can append it to strings
            // ending with the same or earlier vowels
            e += a; // 'e' can be appended to strings ending with 'a'
            i += e; // 'i' can be appended to strings ending with 'a' or 'e'
            o += i; // 'o' can be appended to strings ending with 'a', 'e', or 'i'
            u += o; // 'u' can be appended to strings ending with 'a', 'e', 'i', or 'o'
        }

        // The total count is the sum of all possible strings
        a + e + i + o + u
    }
}
fn main() {}
