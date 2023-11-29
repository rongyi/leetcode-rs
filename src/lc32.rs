struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut stack: Vec<i32> = Vec::new();
        let mut max_len = 0;

        // the index before a valid parentheses
        // the initial imaginary index -1
        stack.push(-1);
        for i in 0..chars.len() {
            match chars[i] {
                '(' => {
                    stack.push(i as i32);
                }
                _ => {
                    stack.pop();
                    if stack.is_empty() {
                        stack.push(i as i32);
                    } else {
                        let cur = i as i32 - *stack.last().unwrap();
                        max_len = max_len.max(cur);
                    }
                }
            }
        }

        max_len
    }
}
