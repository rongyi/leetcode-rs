#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_parentheses(s1: String) -> String {
        let mut s: Vec<char> = s1.chars().collect();
        let mut pair = Vec::new();
        for (i, c) in s1.chars().enumerate() {
            if c == '(' {
                pair.push(i);
            } else if c == ')' {
                // revser in this range,
                // and the range can be calculated by stack's top to here
                let start = pair.pop().unwrap();
                s[start..=i].reverse();
            }
        }

        s.into_iter().filter(|&x| x != '(' && x != ')').collect()
    }
}

fn main() {
    // let s = "(u(love)i)".to_string();
    let s = "a(bcdefghijkl(mno)p)q".to_string();
    let out = Solution::reverse_parentheses(s);
    println!("{}", out);
}
