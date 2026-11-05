struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];

        for c in s.chars() {
            if c == '(' {
                stack.push(0);
            } else {
                let val = stack.pop().unwrap();
                let parent = stack.last_mut().unwrap();
                *parent += 1.max(2 * val);
            }
        }

        stack[0]
    }
}

fn main() {}
