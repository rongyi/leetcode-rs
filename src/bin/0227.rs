struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = vec![];
        let mut sign = '+';
        let mut cur_num = 0;

        for (i, c) in s.chars().enumerate() {
            if c.is_digit(10) {
                cur_num = cur_num * 10 + (c as u8 - '0' as u8) as i32;
            }
            if (!c.is_whitespace() && !c.is_digit(10)) || i == s.len() - 1 {
                match sign {
                    '+' => stack.push(cur_num),
                    '-' => stack.push(-cur_num),
                    '*' => {
                        let prev_num = stack.pop().unwrap();
                        stack.push(prev_num * cur_num);
                    }
                    '/' => {
                        let prev_num = stack.pop().unwrap();
                        stack.push(prev_num / cur_num);
                    }
                    _ => (),
                }
                sign = c;
                cur_num = 0;
            }
        }

        stack.into_iter().sum()
    }
}

fn main() {
    Solution::calculate("5 / 2".to_string());
}
