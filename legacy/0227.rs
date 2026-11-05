struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut sign = '+';
        let mut num = 0;
        let mut stack: Vec<i32> = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }

            if (!c.is_digit(10) && c != ' ') || i == s.len() - 1 {
                match sign {
                    '+' => stack.push(num),
                    '-' => stack.push(-num),
                    '*' => {
                        let prev_num = stack.pop().unwrap();
                        stack.push(prev_num * num);
                    }
                    '/' => {
                        let prev_num = stack.pop().unwrap();
                        stack.push(prev_num / num);
                    }
                    _ => (),
                }
                num = 0;
                sign = c;
            }
        }

        stack.into_iter().sum()
    }
}

fn main() {}
