struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut acp = a.clone();
        let mut ret = 1;
        loop {
            if acp.find(&b).is_some() {
                return ret;
            }

            ret += 1;
            acp.push_str(&a);

            if acp.len() >= b.len() * 2 {
                break;
            }
        }

        if acp.find(&b).is_some() {
            return ret;
        }
        -1
    }
}

fn main() {}
