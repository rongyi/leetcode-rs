#![allow(dead_code)]

struct Solution;

impl Solution {
    // its like zuma game
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();
        for c in s.chars() {
            if let Some((top, cnt)) = stack.last_mut() {
                if *top == c {
                    *cnt += 1;
                    if *cnt == k {
                        stack.pop();
                    }
                } else {
                    stack.push((c, 1));
                }
            } else {
                stack.push((c, 1));
            }
        }
        let mut ret = String::new();
        for (c, cnt) in stack.into_iter() {
            ret.push_str(&c.to_string().repeat(cnt as usize));
        }

        ret
    }
}

fn main() {}
