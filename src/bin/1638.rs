#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut result = 0;

        for i in 0..s.len() {
            for j in 0..t.len() {
                let mut diff = 0;

                for k in 0..s.len().min(t.len()) {
                    if i + k >= s.len() || j + k >= t.len() {
                        break;
                    }

                    if s[i + k] != t[j + k] {
                        diff += 1;
                    }

                    if diff > 1 {
                        break;
                    }

                    if diff == 1 {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

fn main() {}
