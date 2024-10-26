struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut balance = 0;
        let mut ret = String::new();

        for c in s.chars() {
            if c == '(' {
                if balance > 0 {
                    ret.push(c);
                }
                balance += 1;
            } else {
                balance -= 1;
                if balance > 0 {
                    ret.push(c);
                }
            }
        }

        ret
    }
}

fn main() {}
