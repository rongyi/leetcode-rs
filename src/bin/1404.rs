#![allow(dead_code)]

struct Solution;

impl Solution {
    // length is about 500 length
    // so convert to interger is not a valid solution
    pub fn num_steps(s: String) -> i32 {
        let mut s: Vec<char> = s.chars().collect();
        let mut steps = 0;
        while s.len() > 1 {
            // odd
            if s[s.len() - 1] == '0' {
                s.pop();
            } else {
                // even
                // try add 1
                let mut i = s.len() as i32 - 1;
                while i >= 0 && s[i as usize] == '1' {
                    s[i as usize] = '0';
                    i -= 1;
                }
                if i >= 0 {
                    s[i as usize] = '1';
                } else {
                    s.insert(0, '1');
                }
            }
            steps += 1;
        }

        steps
    }
}

fn main() {}
