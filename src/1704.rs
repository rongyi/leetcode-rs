#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.to_lowercase();
        let s_bytes = s.as_bytes();
        let n = s.len();
        let mid = n / 2;

        let vowels = |c: u8| -> bool { matches!(c, b'a' | b'e' | b'i' | b'o' | b'u') };

        let mut first_half_vowels = 0;
        let mut second_half_vowels = 0;

        for i in 0..mid {
            if vowels(s_bytes[i]) {
                first_half_vowels += 1;
            }
            if vowels(s_bytes[i + mid]) {
                second_half_vowels += 1;
            }
        }

        first_half_vowels == second_half_vowels
    }
}

fn main() {}
