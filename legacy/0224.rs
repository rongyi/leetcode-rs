struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stk: Vec<(i32, i32)> = Vec::new();

        let mut sum = 0;
        let mut sign = 1;

        let s: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < s.len() {
            if s[i].is_digit(10) {
                let mut num = 0;
                let mut j = i;
                while j < s.len() && s[j].is_digit(10) {
                    num = num * 10 + (s[j] as i32 - '0' as i32);
                    j += 1;
                }
                // remember the last i += 1
                // we make it one step back
                i = j - 1;
                sum += num * sign;
                sign = 1;
            } else if s[i] == '-' {
                sign = -sign;
            } else if s[i] == '(' {
                stk.push((sum, sign));
                sum = 0;
                sign = 1;
            } else if s[i] == ')' {
                let (prev_sum, prev_sign) = stk.pop().unwrap();
                sum = prev_sum + prev_sign * sum;
            }

            i += 1;
        }

        sum
    }
}

fn main() {}
