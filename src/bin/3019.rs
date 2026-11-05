struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let s = s.to_lowercase();
        let mut changes = 0;

        for i in 1..s.len() {
            if s.as_bytes()[i] != s.as_bytes()[i - 1] {
                changes += 1;
            }
        }

        changes
    }
}

fn main() {}
