#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut a: Vec<char> = Vec::new();
        let mut b = Vec::new();
        for c in s.chars() {
            if c == '#' {
                a.pop();
            } else {
                a.push(c);
            }
        }
        for c in t.chars() {
            if c == '#' {
                b.pop();
            } else {
                b.push(c);
            }
        }

        a.eq(&b)
    }
}

fn main() {}
