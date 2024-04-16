#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut acp = a.clone();
        let mut ret = 1;
        // 1. when b is so small, that we only need double a, if we can not find b in doubled a thats it
        // 2. when b.len > a.len, we need this time to let a's len catch up b len
        // then we double it to search that
        // so both case we get condition:
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
