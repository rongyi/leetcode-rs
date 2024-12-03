#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn last_substring(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        // largest lexiographical start
        let mut i = 0;
        // candicate start
        let mut j = 1;
        // len
        let mut k = 0;
        while j + k < sz {
            if s[i + k] == s[j + k] {
                k += 1;
            } else if s[i + k] < s[j + k] {
                i = i + k + 1;
                k = 0;
                if i >= j {
                    j = i + 1;
                }
            } else {
                j = j + k + 1;
                k = 0;
            }
        }
        s[i..].iter().collect()
    }
}

fn main() {}
