struct Solution;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == 'i' {
                stack.reverse();
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}

fn main() {}
