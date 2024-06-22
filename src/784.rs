#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ret = Vec::new();
        let mut s: Vec<char> = s.chars().collect();

        Self::recur(&mut s, &mut ret, 0);

        ret
    }
    fn recur(s: &mut Vec<char>, ret: &mut Vec<String>, idx: usize) {
        if idx == s.len() {
            ret.push(s.iter().collect());
            return;
        }
        // pick current case or just ignore digit
        Self::recur(s, ret, idx + 1);

        if s[idx].is_alphabetic() {
            // change case
            if s[idx].is_lowercase() {
                s[idx] = ('A' as u8 + s[idx] as u8 - 'a' as u8) as char;
            } else {
                s[idx] = ('a' as u8 + s[idx] as u8 - 'A' as u8) as char;
            }
            Self::recur(s, ret, idx + 1);
            // restore
            if s[idx].is_lowercase() {
                s[idx] = ('A' as u8 + s[idx] as u8 - 'a' as u8) as char;
            } else {
                s[idx] = ('a' as u8 + s[idx] as u8 - 'A' as u8) as char;
            }
        }
    }
}

fn main() {}
