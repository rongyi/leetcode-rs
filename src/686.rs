#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut acp = a.clone();
        let mut ret = 1;
        while ret <= (b.len() / a.len()) as i32 + 2 {
            if acp.find(&b).is_some() {
                return ret;
            }

            ret += 1;
            acp.push_str(&a);
        }

        -1
    }
}

fn main() {}
