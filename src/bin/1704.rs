#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.to_lowercase();
        let s_bytes = s.as_bytes();
        let n = s.len();
        let mid = n / 2;

        let a = s_bytes[0..mid]
            .iter()
            .filter(|&&c| c == b'a' || c == b'o' || c == b'e' || c == b'i' || c == b'u')
            .count();
        let b = s_bytes[mid..]
            .iter()
            .filter(|&&c| c == b'a' || c == b'o' || c == b'e' || c == b'i' || c == b'u')
            .count();

        a == b
    }
}

fn main() {}
