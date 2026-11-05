#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '?' {
                // Try 'a', 'b', 'c' and choose the first that doesn't conflict
                for replacement in ['a', 'b', 'c'] {
                    let left_ok = i == 0 || chars[i - 1] != replacement;
                    let right_ok = i == chars.len() - 1 || chars[i + 1] != replacement;

                    if left_ok && right_ok {
                        chars[i] = replacement;
                        break;
                    }
                }
            }
        }

        chars.into_iter().collect()
    }
}

fn main() {}
