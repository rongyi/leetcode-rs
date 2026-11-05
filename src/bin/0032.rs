struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let mut stack = vec![-1];
        let mut ret = 0;
        for (i, &c) in s.iter().enumerate() {
            if c == b'(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let cur_len = i as i32 - *stack.last().unwrap();
                    ret = ret.max(cur_len);
                }
            }
        }

        ret
    }
}

fn main() {}
