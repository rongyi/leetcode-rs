#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let mut stack = Vec::new();
        for (i, &c) in s.clone().iter().enumerate() {
            if c == '(' {
                stack.push(i);
            } else if c == ')' {
                if stack.is_empty() {
                    // lonely ')'
                    s[i] = '*';
                } else {
                    // a matched pair
                    stack.pop();
                }
            }
        }
        // now in stack are the lonely '(' with no right match
        for idx in stack.into_iter() {
            s[idx] = '*'
        }

        s.into_iter().filter(|&c| c != '*').collect()
    }
}

fn main() {}
