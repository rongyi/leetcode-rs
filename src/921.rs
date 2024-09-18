struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut stk: Vec<char> = Vec::new();

        for c in s.chars() {
            if c == '(' {
                stk.push('(');
            } else {
                if stk.is_empty() {
                    left += 1;
                } else {
                    stk.pop();
                }
            }
        }
        right = stk.len() as i32;

        left + right
    }
}

fn main() {}
