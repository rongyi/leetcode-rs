struct Solution;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for i in 1..=n {
            let cur = format!("{:b}", i);
            if !s.contains(&cur) {
                return false;
            }
        }
        true
    }
}

fn main() {}
