struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut out = String::new();
        for c in s.chars() {
            if c == '*' {
                out.pop();
            } else {
                out.push(c);
            }
        }

        out
    }
}

fn main() {}
